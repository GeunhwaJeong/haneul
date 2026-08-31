// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use anyhow::{anyhow, Result};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use shared_crypto::intent::{Intent, IntentMessage};
use haneul_sdk::rpc_types::{
    HaneulObjectDataFilter, HaneulObjectDataOptions, HaneulObjectResponseQuery, HaneulTransactionBlockResponse,
    HaneulTransactionBlockResponseOptions,
};
use haneul_sdk::types::base_types::{ObjectID, ObjectRef, HaneulAddress};
use haneul_sdk::types::coin::{COIN_MODULE_NAME, COIN_TREASURE_CAP_NAME};
use haneul_sdk::types::crypto::{Signature, HaneulKeyPair};
use haneul_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use haneul_sdk::types::transaction_driver_types::ExecuteTransactionRequestType;
use haneul_sdk::types::transaction::{Argument, Command, ObjectArg, Transaction, TransactionData};
use haneul_sdk::types::{HANEUL_FRAMEWORK_ADDRESS, HANEUL_FRAMEWORK_PACKAGE_ID};
use haneul_sdk::HaneulClient;
use tracing::info;

use crate::gas::select_gas;

pub async fn get_treasury_cap(
    client: &HaneulClient,
    owner_addr: HaneulAddress,
    type_tag: TypeTag,
) -> Result<ObjectRef> {
    let resp = client
        .read_api()
        .get_owned_objects(
            owner_addr,
            Some(HaneulObjectResponseQuery {
                filter: Some(HaneulObjectDataFilter::StructType(StructTag {
                    address: HANEUL_FRAMEWORK_ADDRESS,
                    module: Identifier::from(COIN_MODULE_NAME),
                    name: Identifier::from(COIN_TREASURE_CAP_NAME),
                    type_params: vec![type_tag],
                })),
                options: None,
            }),
            None,
            None,
        )
        .await?;

    let treasury_cap = resp
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No deny-cap found!"))?;
    Ok(treasury_cap
        .data
        .ok_or(anyhow!("DenyCap empty!"))?
        .object_ref())
}

pub async fn get_coin(client: &HaneulClient, id: ObjectID) -> Result<ObjectRef> {
    let resp = client
        .read_api()
        .get_object_with_options(
            id,
            HaneulObjectDataOptions {
                // Note that we could have the type-tag from here and transfer in a moment
                show_type: false,
                show_owner: false,
                show_previous_transaction: false,
                show_display: false,
                show_content: false,
                show_bcs: false,
                show_storage_rebate: false,
            },
        )
        .await?;

    Ok(resp.data.ok_or(anyhow!("No coin found"))?.object_ref())
}

// docs::#mint
pub async fn mint_and_transfer(
    client: &HaneulClient,
    signer: &HaneulKeyPair,
    type_tag: TypeTag,
    treasury_cap: ObjectRef,
    to_address: HaneulAddress,
    balance: u64,
) -> Result<HaneulTransactionBlockResponse> {
    info!("MINTING COIN OF BALANCE {balance} TO ADDRESS {to_address}");
    let signer_addr = HaneulAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let treasury_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(treasury_cap))?;
    let balance = ptb.pure(balance)?;
    ptb.command(Command::move_call(
        HANEUL_FRAMEWORK_PACKAGE_ID,
        Identifier::from(COIN_MODULE_NAME),
        Identifier::from_str("mint")?,
        vec![type_tag],
        vec![treasury_cap, balance],
    ));
    ptb.transfer_arg(to_address, Argument::Result(0));

    let builder = ptb.finish();

    // Sign transaction
    let msg = IntentMessage {
        intent: Intent::haneul_transaction(),
        value: TransactionData::new_programmable(
            signer_addr,
            vec![gas_data.object],
            builder,
            gas_data.budget,
            gas_data.price,
        ),
    };
    let sig = Signature::new_secure(&msg, signer);

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(msg.value, vec![sig]),
            HaneulTransactionBlockResponseOptions::new()
                .with_effects()
                .with_object_changes()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)
}
// docs::/#mint

pub async fn transfer(
    client: &HaneulClient,
    signer: &HaneulKeyPair,
    coin: ObjectRef,
    to_address: HaneulAddress,
) -> Result<HaneulTransactionBlockResponse> {
    info!("TRANSFERRING COIN {} TO ADDRESS {to_address}", coin.0);
    let signer_addr = HaneulAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let coin = ptb.obj(ObjectArg::ImmOrOwnedObject(coin))?;
    ptb.transfer_arg(to_address, coin);

    let builder = ptb.finish();

    // Sign transaction
    let msg = IntentMessage {
        intent: Intent::haneul_transaction(),
        value: TransactionData::new_programmable(
            signer_addr,
            vec![gas_data.object],
            builder,
            gas_data.budget,
            gas_data.price,
        ),
    };
    let sig = Signature::new_secure(&msg, signer);

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(msg.value, vec![sig]),
            HaneulTransactionBlockResponseOptions::new()
                .with_effects()
                .with_object_changes()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)
}

pub(crate) async fn burn(
    client: &HaneulClient,
    signer: &HaneulKeyPair,
    type_tag: TypeTag,
    treasury_cap: ObjectRef,
    coin: ObjectRef,
) -> Result<HaneulTransactionBlockResponse> {
    info!("BURNING COIN {}", coin.0);
    let signer_addr = HaneulAddress::from(&signer.public());
    let gas_data = select_gas(client, signer_addr, None, None, vec![], None).await?;

    let mut ptb = ProgrammableTransactionBuilder::new();

    let treasury_cap = ptb.obj(ObjectArg::ImmOrOwnedObject(treasury_cap))?;
    let coin = ptb.obj(ObjectArg::ImmOrOwnedObject(coin))?;
    ptb.command(Command::move_call(
        HANEUL_FRAMEWORK_PACKAGE_ID,
        Identifier::from(COIN_MODULE_NAME),
        Identifier::from_str("burn")?,
        vec![type_tag],
        vec![treasury_cap, coin],
    ));

    let builder = ptb.finish();

    // Sign transaction
    let msg = IntentMessage {
        intent: Intent::haneul_transaction(),
        value: TransactionData::new_programmable(
            signer_addr,
            vec![gas_data.object],
            builder,
            gas_data.budget,
            gas_data.price,
        ),
    };
    let sig = Signature::new_secure(&msg, signer);

    let res = client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(msg.value, vec![sig]),
            HaneulTransactionBlockResponseOptions::new()
                .with_effects()
                .with_input(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(res)

}
