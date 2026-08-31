// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

mod client;
mod restore;
mod v2;

async fn transfer_coin(
    context: &haneul_sdk::wallet_context::WalletContext,
) -> haneul_sdk_types::Digest {
    let gas_price = context.get_reference_gas_price().await.unwrap();
    let accounts_and_objs = context.get_all_accounts_and_gas_objects().await.unwrap();
    let sender = accounts_and_objs[0].0;
    let receiver = accounts_and_objs[1].0;
    let gas_object = accounts_and_objs[0].1[0];
    let object_to_send = accounts_and_objs[0].1[1];
    let txn = context
        .sign_transaction(
            &haneul_test_transaction_builder::TestTransactionBuilder::new(
                sender, gas_object, gas_price,
            )
            .transfer(
                haneul_types::base_types::FullObjectRef::from_fastpath_ref(object_to_send),
                receiver,
            )
            .build(),
        )
        .await;
    let resp = context.execute_transaction_must_succeed(txn).await;
    resp.transaction.digest().into()
}

async fn stake_with_validator(cluster: &test_cluster::TestCluster) -> haneul_sdk_types::Digest {
    let context = &cluster.wallet;
    let gas_price = context.get_reference_gas_price().await.unwrap();
    let accounts_and_objs = context.get_all_accounts_and_gas_objects().await.unwrap();
    let sender = accounts_and_objs[0].0;
    let gas_object = accounts_and_objs[0].1[0];
    let coin_to_stake = accounts_and_objs[0].1[1];
    let validator_address = cluster.swarm.config().validator_configs()[0].haneul_address();
    let txn = context
        .sign_transaction(
            &haneul_test_transaction_builder::TestTransactionBuilder::new(
                sender, gas_object, gas_price,
            )
            .call_staking(coin_to_stake, validator_address)
            .build(),
        )
        .await;
    let resp = context.execute_transaction_must_succeed(txn).await;
    resp.transaction.digest().into()
}
