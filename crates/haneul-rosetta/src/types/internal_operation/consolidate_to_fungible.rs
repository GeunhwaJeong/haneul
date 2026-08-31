// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use futures::TryStreamExt;
use haneul_rpc::client::Client;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2::{GetEpochRequest, ListOwnedObjectsRequest};
use haneul_sdk_types::Address;
use haneul_types::base_types::{HaneulAddress, ObjectID, ObjectRef};
use haneul_types::haneul_system_state::HANEUL_SYSTEM_MODULE_NAME;
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::rpc_proto_conversions::ObjectReferenceExt;
use haneul_types::transaction::{CallArg, Command, ObjectArg, ProgrammableTransaction};
use haneul_types::{HANEUL_SYSTEM_PACKAGE_ID, Identifier};
use prost_types::FieldMask;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::errors::Error;

use super::{TransactionObjectData, TryConstructTransaction, simulate_transaction};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ConsolidateAllStakedHaneulToFungible {
    pub sender: HaneulAddress,
    pub validator: HaneulAddress,
}

/// BCS layout for `0x3::staking_pool::StakedHaneul`.
/// Field order must match the Move struct definition exactly (BCS is positional).
/// See: crates/haneul-framework/packages/haneul-system/sources/staking_pool.move
#[derive(Deserialize)]
struct StakedHaneulBcs {
    _id: Address,
    pool_id: Address,
    stake_activation_epoch: u64,
    _principal: u64,
}

/// BCS layout for `0x3::staking_pool::FungibleStakedHaneul`.
/// Field order must match the Move struct definition exactly (BCS is positional).
/// See: crates/haneul-framework/packages/haneul-system/sources/staking_pool.move
#[derive(Deserialize)]
struct FungibleStakedHaneulBcs {
    _id: Address,
    pool_id: Address,
    _value: u64,
}

#[async_trait]
impl TryConstructTransaction for ConsolidateAllStakedHaneulToFungible {
    async fn try_fetch_needed_objects(
        self,
        client: &mut Client,
        gas_price: Option<u64>,
        budget: Option<u64>,
    ) -> Result<TransactionObjectData, Error> {
        let Self { sender, validator } = self;

        let current_epoch = crate::get_current_epoch(client).await?;
        let pool_id = get_validator_pool_id(client, validator).await?;

        let staked_haneul_refs =
            discover_staked_haneul(client, sender, &pool_id, current_epoch).await?;
        let fss_refs = discover_fss(client, sender, &pool_id).await?;

        if staked_haneul_refs.is_empty() && fss_refs.len() <= 1 {
            return Err(Error::InvalidInput(format!(
                "Nothing to consolidate for validator {}: {} activated StakedHaneul, {} FungibleStakedHaneul",
                validator,
                staked_haneul_refs.len(),
                fss_refs.len(),
            )));
        }

        let total_commands = staked_haneul_refs.len() * 2 + fss_refs.len() + 2;
        if total_commands > super::MAX_COMMAND_ARGS {
            return Err(Error::InvalidInput(format!(
                "Too many objects to consolidate ({} StakedHaneul + {} FSS). Maximum ~{} objects supported.",
                staked_haneul_refs.len(),
                fss_refs.len(),
                super::MAX_COMMAND_ARGS / 2,
            )));
        }

        let fss_count = fss_refs.len();

        // Objects layout: FSS refs first, then StakedHaneul refs
        let mut all_objects = Vec::with_capacity(fss_refs.len() + staked_haneul_refs.len());
        all_objects.extend_from_slice(&fss_refs);
        all_objects.extend_from_slice(&staked_haneul_refs);

        let pt = consolidate_to_fungible_pt(sender, fss_refs, staked_haneul_refs.clone())?;
        let (budget, gas_coin_objs) =
            simulate_transaction(client, pt, sender, vec![], gas_price, budget).await?;

        let total_haneul_balance = gas_coin_objs.iter().map(|c| c.balance()).sum::<u64>() as i128;
        let gas_coins = gas_coin_objs
            .iter()
            .map(|obj| obj.object_reference().try_to_object_ref())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TransactionObjectData {
            gas_coins,
            objects: all_objects,
            party_objects: vec![],
            total_haneul_balance,
            budget,
            address_balance_withdrawal: 0,
            fss_object_count: Some(fss_count as u64),
            redeem_token_amount: None,
            redeem_plan: None,
            bind_epoch: None,
        })
    }
}

async fn discover_staked_haneul(
    client: &mut Client,
    sender: HaneulAddress,
    pool_id: &str,
    current_epoch: u64,
) -> Result<Vec<ObjectRef>, Error> {
    let list_request = ListOwnedObjectsRequest::default()
        .with_owner(sender.to_string())
        .with_object_type("0x3::staking_pool::StakedHaneul".to_string())
        .with_page_size(1000u32)
        .with_read_mask(FieldMask::from_paths([
            "object_id",
            "version",
            "digest",
            "contents",
        ]));

    let objects: Vec<_> = client
        .list_owned_objects(list_request)
        .map_err(Error::from)
        .try_collect()
        .await?;

    let mut refs = Vec::new();
    for obj in objects {
        let contents = obj
            .contents
            .as_ref()
            .ok_or_else(|| Error::DataError("StakedHaneul missing contents".to_string()))?;
        let staked: StakedHaneulBcs = contents
            .deserialize()
            .map_err(|e| Error::DataError(format!("Failed to deserialize StakedHaneul: {}", e)))?;

        if staked.pool_id.to_string() == pool_id && current_epoch >= staked.stake_activation_epoch {
            refs.push((
                ObjectID::from_str(obj.object_id())
                    .map_err(|e| Error::DataError(format!("Invalid object_id: {}", e)))?,
                obj.version().into(),
                obj.digest()
                    .parse()
                    .map_err(|e| Error::DataError(format!("Invalid digest: {}", e)))?,
            ));
        }
    }
    Ok(refs)
}

async fn discover_fss(
    client: &mut Client,
    sender: HaneulAddress,
    pool_id: &str,
) -> Result<Vec<ObjectRef>, Error> {
    let list_request = ListOwnedObjectsRequest::default()
        .with_owner(sender.to_string())
        .with_object_type("0x3::staking_pool::FungibleStakedHaneul".to_string())
        .with_page_size(1000u32)
        .with_read_mask(FieldMask::from_paths([
            "object_id",
            "version",
            "digest",
            "contents",
        ]));

    let objects: Vec<_> = client
        .list_owned_objects(list_request)
        .map_err(Error::from)
        .try_collect()
        .await?;

    let mut refs = Vec::new();
    for obj in objects {
        let contents = obj
            .contents
            .as_ref()
            .ok_or_else(|| Error::DataError("FungibleStakedHaneul missing contents".to_string()))?;
        let fss: FungibleStakedHaneulBcs = contents.deserialize().map_err(|e| {
            Error::DataError(format!("Failed to deserialize FungibleStakedHaneul: {}", e))
        })?;

        if fss.pool_id.to_string() == pool_id {
            refs.push((
                ObjectID::from_str(obj.object_id())
                    .map_err(|e| Error::DataError(format!("Invalid object_id: {}", e)))?,
                obj.version().into(),
                obj.digest()
                    .parse()
                    .map_err(|e| Error::DataError(format!("Invalid digest: {}", e)))?,
            ));
        }
    }
    Ok(refs)
}

pub(crate) async fn get_validator_pool_id(
    client: &mut Client,
    validator: HaneulAddress,
) -> Result<String, Error> {
    let request = GetEpochRequest::latest().with_read_mask(FieldMask::from_paths([
        "system_state.validators.active_validators",
    ]));
    let response = client
        .ledger_client()
        .get_epoch(request)
        .await?
        .into_inner();
    let validators = response
        .epoch()
        .system_state()
        .validators()
        .active_validators();

    for v in validators {
        if let Ok(addr) = v.address().parse::<HaneulAddress>()
            && addr == validator
        {
            return Ok(v.staking_pool().id().to_string());
        }
    }
    Err(Error::InvalidInput(format!(
        "Validator {} not found in active validators",
        validator
    )))
}

/// Build PTB for consolidating StakedHaneul → FungibleStakedHaneul.
///
/// Phase 1: Merge existing FSS (if >1)
/// Phase 2: Convert each StakedHaneul → FSS
/// Phase 3: Merge all new FSS together (if >1)
/// Phase 4: Merge new into existing (if existing) or TransferObjects to sender
pub fn consolidate_to_fungible_pt(
    sender: HaneulAddress,
    fss_refs: Vec<ObjectRef>,
    staked_haneul_refs: Vec<ObjectRef>,
) -> anyhow::Result<ProgrammableTransaction> {
    let mut builder = ProgrammableTransactionBuilder::new();

    if fss_refs.is_empty() && staked_haneul_refs.is_empty() {
        return Ok(builder.finish());
    }

    let system_state = builder.input(CallArg::HANEUL_SYSTEM_MUT)?;

    // Phase 1: Merge existing FSS into the first one using staking_pool::join_fungible_staked_haneul
    // MergeCoins only works on Coin<T>, not FungibleStakedHaneul
    let existing_fss = if !fss_refs.is_empty() {
        let first = builder.obj(ObjectArg::ImmOrOwnedObject(fss_refs[0]))?;
        for fss_ref in &fss_refs[1..] {
            let other = builder.obj(ObjectArg::ImmOrOwnedObject(*fss_ref))?;
            builder.command(Command::move_call(
                HANEUL_SYSTEM_PACKAGE_ID,
                Identifier::new("staking_pool")?,
                Identifier::new("join_fungible_staked_haneul")?,
                vec![],
                vec![first, other],
            ));
        }
        Some(first)
    } else {
        None
    };

    // Phase 2: Convert each StakedHaneul → FSS
    let mut new_fss_results = Vec::new();
    for staked_ref in &staked_haneul_refs {
        let staked_haneul_arg = builder.obj(ObjectArg::ImmOrOwnedObject(*staked_ref))?;
        let result = builder.command(Command::move_call(
            HANEUL_SYSTEM_PACKAGE_ID,
            HANEUL_SYSTEM_MODULE_NAME.to_owned(),
            Identifier::new("convert_to_fungible_staked_haneul")?,
            vec![],
            vec![system_state, staked_haneul_arg],
        ));
        new_fss_results.push(result);
    }

    // Phase 3: Merge all new FSS together using join_fungible_staked_haneul
    if new_fss_results.len() > 1 {
        for i in 1..new_fss_results.len() {
            builder.command(Command::move_call(
                HANEUL_SYSTEM_PACKAGE_ID,
                Identifier::new("staking_pool")?,
                Identifier::new("join_fungible_staked_haneul")?,
                vec![],
                vec![new_fss_results[0], new_fss_results[i]],
            ));
        }
    }

    // Phase 4: Merge into existing or transfer to sender
    if let Some(existing) = existing_fss {
        if !new_fss_results.is_empty() {
            builder.command(Command::move_call(
                HANEUL_SYSTEM_PACKAGE_ID,
                Identifier::new("staking_pool")?,
                Identifier::new("join_fungible_staked_haneul")?,
                vec![],
                vec![existing, new_fss_results[0]],
            ));
        }
        // existing FSS is already owned by sender, no transfer needed
    } else if !new_fss_results.is_empty() {
        let sender_arg = builder.pure(sender)?;
        builder.command(Command::TransferObjects(
            vec![new_fss_results[0]],
            sender_arg,
        ));
    }

    Ok(builder.finish())
}
