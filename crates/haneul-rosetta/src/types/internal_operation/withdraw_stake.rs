// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use haneul_rpc::client::Client;
use prost_types::FieldMask;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2::{GetObjectRequest, ListOwnedObjectsRequest};
use haneul_types::HANEUL_SYSTEM_PACKAGE_ID;
use haneul_types::base_types::{HaneulAddress, ObjectID, ObjectRef};
use haneul_types::governance::WITHDRAW_STAKE_FUN_NAME;
use haneul_types::haneul_system_state::HANEUL_SYSTEM_MODULE_NAME;
use haneul_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_types::rpc_proto_conversions::ObjectReferenceExt;
use haneul_types::transaction::{CallArg, Command, ObjectArg, ProgrammableTransaction};

use crate::errors::Error;

use super::{TransactionObjectData, TryConstructTransaction, simulate_transaction};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WithdrawStake {
    pub sender: HaneulAddress,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stake_ids: Vec<ObjectID>,
}

#[async_trait]
impl TryConstructTransaction for WithdrawStake {
    async fn try_fetch_needed_objects(
        self,
        client: &mut Client,
        gas_price: Option<u64>,
        budget: Option<u64>,
    ) -> Result<TransactionObjectData, Error> {
        let Self { sender, stake_ids } = self;

        let withdraw_all = stake_ids.is_empty();
        let stake_ids = if withdraw_all {
            use futures::TryStreamExt;

            let list_request = ListOwnedObjectsRequest::default()
                .with_owner(sender.to_string())
                .with_object_type("0x3::staking_pool::StakedHaneul".to_string())
                .with_page_size(1000u32)
                .with_read_mask(FieldMask::from_paths(["object_id"]));

            client
                .list_owned_objects(list_request)
                .map_err(Error::from)
                .and_then(|object| async move {
                    let object_id = ObjectID::from_hex_literal(object.object_id())
                        .map_err(|e| Error::InvalidInput(format!("Invalid object_id: {}", e)))?;
                    Ok(object_id)
                })
                .try_collect::<Vec<_>>()
                .await?
        } else {
            stake_ids
        };

        if stake_ids.is_empty() {
            return Err(Error::InvalidInput("No active stake to withdraw".into()));
        }

        let mut stake_refs = Vec::new();
        for stake_id in &stake_ids {
            let stake_address: haneul_sdk_types::Address = (*stake_id).into();
            let request = GetObjectRequest::new(&stake_address)
                .with_read_mask(FieldMask::from_paths(["object_id", "version", "digest"]));

            let response = client
                .ledger_client()
                .get_object(request)
                .await?
                .into_inner();

            if let Some(object) = response.object {
                let object_id = object.object_id.ok_or_else(|| {
                    Error::InvalidInput("Object ID missing in response".to_string())
                })?;
                let version = object.version.ok_or_else(|| {
                    Error::InvalidInput("Version missing in response".to_string())
                })?;
                let digest = object
                    .digest
                    .ok_or_else(|| Error::InvalidInput("Digest missing in response".to_string()))?;

                stake_refs.push((
                    ObjectID::from_str(&object_id).map_err(|e| {
                        Error::InvalidInput(format!("Failed to parse object ID: {}", e))
                    })?,
                    version.into(),
                    digest.parse().map_err(|e| {
                        Error::InvalidInput(format!("Failed to parse digest: {}", e))
                    })?,
                ));
            } else {
                return Err(Error::InvalidInput(format!(
                    "Stake object {} not found",
                    stake_id
                )));
            }
        }

        let pt = withdraw_stake_pt(stake_refs.clone(), withdraw_all)?;
        let (budget, gas_coin_objs) =
            simulate_transaction(client, pt, sender, vec![], gas_price, budget).await?;

        let total_haneul_balance = gas_coin_objs.iter().map(|c| c.balance()).sum::<u64>() as i128;
        let gas_coins = gas_coin_objs
            .iter()
            .map(|obj| obj.object_reference().try_to_object_ref())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TransactionObjectData {
            gas_coins,
            objects: stake_refs,
            party_objects: vec![],
            total_haneul_balance,
            budget,
            address_balance_withdrawal: 0,
            fss_object_count: None,
            redeem_token_amount: None,
            redeem_plan: None,
            bind_epoch: None,
        })
    }
}

pub fn withdraw_stake_pt(
    stake_objs: Vec<ObjectRef>,
    withdraw_all: bool,
) -> anyhow::Result<ProgrammableTransaction> {
    let mut builder = ProgrammableTransactionBuilder::new();

    for stake_id in stake_objs {
        // [WORKAROUND] - this is a hack to work out if the withdraw stake ops is for selected stake_ids or None (all stakes) using the index of the call args.
        // if stake_ids is not empty, id input will be created after the system object input
        // TODO: Investigate whether using asimple input argument with relevant metadata, similar
        // to PayCoinOperation, would work as well or even better. Would help with consistency.
        let (system_state, id) = if !withdraw_all {
            let system_state = builder.input(CallArg::HANEUL_SYSTEM_MUT)?;
            let id = builder.obj(ObjectArg::ImmOrOwnedObject(stake_id))?;
            (system_state, id)
        } else {
            let id = builder.obj(ObjectArg::ImmOrOwnedObject(stake_id))?;
            let system_state = builder.input(CallArg::HANEUL_SYSTEM_MUT)?;
            (system_state, id)
        };

        let arguments = vec![system_state, id];
        builder.command(Command::move_call(
            HANEUL_SYSTEM_PACKAGE_ID,
            HANEUL_SYSTEM_MODULE_NAME.to_owned(),
            WITHDRAW_STAKE_FUN_NAME.to_owned(),
            vec![],
            arguments,
        ));
    }
    Ok(builder.finish())
}
