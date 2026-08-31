// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::abi::{EthHaneulBridge, eth_haneul_bridge};
use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::e2e_tests::test_utils::{
    BridgeTestClusterBuilder, TestClusterWrapperBuilder, get_signatures,
    initiate_bridge_erc20_to_haneul, initiate_bridge_eth_to_haneul,
    initiate_bridge_eth_to_haneul_v2, initiate_bridge_haneul_to_eth,
    initiate_bridge_haneul_to_eth_v2, send_eth_tx_and_get_tx_receipt, wait_for_eth_coin_owned_by,
};
use crate::encoding::TOKEN_TRANSFER_MESSAGE_VERSION_V2;
use crate::eth_transaction_builder::build_eth_transaction;
use crate::events::{
    HaneulBridgeEvent, HaneulToEthTokenBridgeV1, HaneulToEthTokenBridgeV2, TokenTransferApproved,
    TokenTransferClaimed,
};
use crate::haneul_transaction_builder::build_add_tokens_on_haneul_transaction;
use crate::types::{AddTokensOnEvmAction, BridgeAction};
use crate::utils::publish_and_register_coins_return_add_coins_on_haneul_action;
use alloy::primitives::{Address as EthAddress, U256};
use alloy::providers::Provider;
use haneul_types::bridge::{
    BridgeChainId, BridgeTokenMetadata, BridgeTrait, TOKEN_ID_ETH, get_bridge,
};
use haneul_types::coin::Coin;
use haneul_types::crypto::get_key_pair;
use haneul_types::effects::TransactionEffectsAPI;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use tracing::info;

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_from_eth_to_haneul_to_eth() {
    telemetry_subscribers::init_for_testing();

    let eth_chain_id = BridgeChainId::EthCustom as u8;
    let haneul_chain_id = BridgeChainId::HaneulCustom as u8;
    let timer = std::time::Instant::now();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    let haneul_address = bridge_test_cluster.haneul_user_address();
    let amount = 42;
    let haneul_amount = amount * 100_000_000;

    initiate_bridge_eth_to_haneul(&bridge_test_cluster, amount, 0)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            2,
        )
        .await;
    // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);

    let eth_coin = wait_for_eth_coin_owned_by(&bridge_test_cluster, haneul_address, None).await;
    let (_ty, balance) = Coin::extract_balance_if_coin(&eth_coin).unwrap().unwrap();
    assert_eq!(balance, haneul_amount);
    info!(
        "[Timer] Eth to Haneul bridge transfer finished in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    // Now let the recipient send the coin back to ETH
    let eth_address_1 = EthAddress::random();
    let nonce = 0;

    let haneul_to_eth_bridge_action = initiate_bridge_haneul_to_eth(
        &bridge_test_cluster,
        eth_address_1,
        eth_coin.compute_object_reference(),
        nonce,
        haneul_amount,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                HaneulToEthTokenBridgeV1.get().unwrap().clone(),
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            2,
        )
        .await;
    // There are exactly 1 deposit and 1 approved event
    assert_eq!(events.len(), 2);
    info!(
        "[Timer] Haneul to Eth bridge transfer approved in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    // Test `get_parsed_token_transfer_message`
    let parsed_msg = bridge_test_cluster
        .bridge_client()
        .get_parsed_token_transfer_message(haneul_chain_id, nonce)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(parsed_msg.source_chain as u8, haneul_chain_id);
    assert_eq!(parsed_msg.seq_num, nonce);
    assert_eq!(
        parsed_msg.parsed_payload.sender_address,
        haneul_address.to_vec()
    );
    assert_eq!(
        &parsed_msg.parsed_payload.target_address,
        eth_address_1.as_slice()
    );
    assert_eq!(parsed_msg.parsed_payload.target_chain, eth_chain_id);
    assert_eq!(parsed_msg.parsed_payload.token_type, TOKEN_ID_ETH);
    assert_eq!(parsed_msg.parsed_payload.amount, haneul_amount);

    let message: eth_haneul_bridge::BridgeUtils::Message =
        haneul_to_eth_bridge_action.try_into().unwrap();
    let signatures =
        get_signatures(bridge_test_cluster.bridge_client(), nonce, haneul_chain_id).await;

    let (eth_signer, _) = bridge_test_cluster.get_eth_signer_and_address().unwrap();

    let eth_haneul_bridge = EthHaneulBridge::new(
        bridge_test_cluster.contracts().haneul_bridge,
        eth_signer.clone(),
    );
    let call = eth_haneul_bridge.transferBridgedTokensWithSignatures(signatures, message);
    let eth_claim_tx_receipt =
        send_eth_tx_and_get_tx_receipt(eth_signer.clone(), call.into_transaction_request()).await;
    assert!(eth_claim_tx_receipt.status());
    info!(
        "[Timer] Haneul to Eth bridge transfer claimed in {:?}",
        timer.elapsed()
    );
    // Assert eth_address_1 has received ETH
    assert_eq!(
        eth_signer.get_balance(eth_address_1).await.unwrap(),
        U256::from(amount) * U256::from(10).pow(U256::from(18))
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_from_eth_to_haneul_to_eth_v2() {
    telemetry_subscribers::init_for_testing();

    let eth_chain_id = BridgeChainId::EthCustom as u8;
    let haneul_chain_id = BridgeChainId::HaneulCustom as u8;
    let timer = std::time::Instant::now();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );

    // Upgrade the bridge to V2 before testing V2 functionality
    let timer = std::time::Instant::now();
    bridge_test_cluster
        .upgrade_bridge_to_v2()
        .await
        .expect("Failed to upgrade bridge to V2");
    info!("[Timer] Bridge upgraded to V2 in {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    let (eth_signer, _) = bridge_test_cluster.get_eth_signer_and_address().unwrap();

    let haneul_address = bridge_test_cluster.haneul_user_address();
    let amount = 24;
    let haneul_amount = amount * 100_000_000;

    initiate_bridge_eth_to_haneul_v2(&bridge_test_cluster, amount, 0)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            2,
        )
        .await;
    assert_eq!(events.len(), 2);

    let eth_coin = wait_for_eth_coin_owned_by(&bridge_test_cluster, haneul_address, None).await;
    let (_ty, balance) = Coin::extract_balance_if_coin(&eth_coin).unwrap().unwrap();
    assert_eq!(balance, haneul_amount);
    info!(
        "[Timer] Eth to Haneul bridge transfer v2 finished in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    let eth_address_1 = EthAddress::random();
    let nonce = 0;

    let haneul_to_eth_bridge_action = initiate_bridge_haneul_to_eth_v2(
        &bridge_test_cluster,
        eth_address_1,
        eth_coin.compute_object_reference(),
        nonce,
        haneul_amount,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                HaneulToEthTokenBridgeV2.get().unwrap().clone(),
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            2,
        )
        .await;
    assert_eq!(events.len(), 2);
    info!(
        "[Timer] Haneul to Eth bridge transfer v2 approved in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    let parsed_msg = bridge_test_cluster
        .bridge_client()
        .get_parsed_token_transfer_message(haneul_chain_id, nonce)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(parsed_msg.source_chain as u8, haneul_chain_id);
    assert_eq!(parsed_msg.seq_num, nonce);
    assert_eq!(
        parsed_msg.parsed_payload.sender_address,
        haneul_address.to_vec()
    );
    assert_eq!(
        parsed_msg.parsed_payload.target_address,
        eth_address_1.to_vec()
    );
    assert_eq!(parsed_msg.parsed_payload.target_chain, eth_chain_id);
    assert_eq!(parsed_msg.parsed_payload.token_type, TOKEN_ID_ETH);
    assert_eq!(parsed_msg.parsed_payload.amount, haneul_amount);
    assert_eq!(
        parsed_msg.message_version,
        TOKEN_TRANSFER_MESSAGE_VERSION_V2
    );

    let message: eth_haneul_bridge::BridgeUtils::Message =
        haneul_to_eth_bridge_action.try_into().unwrap();
    let signatures =
        get_signatures(bridge_test_cluster.bridge_client(), nonce, haneul_chain_id).await;

    let eth_haneul_bridge = EthHaneulBridge::new(
        bridge_test_cluster.contracts().haneul_bridge,
        eth_signer.clone(),
    );
    let call = eth_haneul_bridge.transferBridgedTokensWithSignaturesV2(signatures, message);
    let eth_claim_tx_receipt =
        send_eth_tx_and_get_tx_receipt(eth_signer.clone(), call.into_transaction_request()).await;
    assert!(eth_claim_tx_receipt.status());
    info!(
        "[Timer] Haneul to Eth bridge transfer v2 claimed in {:?}",
        timer.elapsed()
    );
    assert_eq!(
        eth_signer.get_balance(eth_address_1).await.unwrap(),
        U256::from(amount) * U256::from(10).pow(U256::from(18))
    );
}

// Test add new coins on both Haneul and Eth
// Also test bridge ndoe handling `NewTokenEvent`
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_add_new_coins_on_haneul_and_eth() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    // Register tokens on Haneul
    let token_id = 5;
    let token_haneul_decimal = 9; // this needs to match ka.move
    let token_price = 10000;
    let sender = bridge_test_cluster.haneul_user_address();
    info!("Published new token");
    let haneul_action = publish_and_register_coins_return_add_coins_on_haneul_action(
        bridge_test_cluster.wallet(),
        bridge_arg,
        vec![Path::new("../../bridge/move/tokens/mock/ka").into()],
        vec![token_id],
        vec![token_price],
        1, // seq num
    )
    .await;
    let new_token_erc_address = bridge_test_cluster.contracts().ka;
    let eth_action = BridgeAction::AddTokensOnEvmAction(AddTokensOnEvmAction {
        nonce: 0,
        chain_id: BridgeChainId::EthCustom,
        native: true,
        token_ids: vec![token_id],
        token_addresses: vec![new_token_erc_address],
        token_haneul_decimals: vec![token_haneul_decimal],
        token_prices: vec![token_price],
    });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![haneul_action.clone(), eth_action.clone()],
        vec![haneul_action.clone()],
        vec![eth_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster().await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_haneul_action = agg
        .request_committee_signatures(haneul_action)
        .await
        .expect("Failed to request committee signatures for AddTokensOnHaneulAction");
    let certified_eth_action = agg
        .request_committee_signatures(eth_action.clone())
        .await
        .expect("Failed to request committee signatures for AddTokensOnEvmAction");

    let tx = build_add_tokens_on_haneul_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_haneul_action,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects;
    assert!(effects.status().is_ok());
    assert!(response.events.unwrap().data.iter().any(|e| {
        let haneul_bridge_event = HaneulBridgeEvent::try_from_event(e).unwrap().unwrap();
        match haneul_bridge_event {
            HaneulBridgeEvent::NewTokenEvent(e) => {
                assert_eq!(e.token_id, token_id);
                true
            }
            _ => false,
        }
    }));
    info!("Approved new token on Haneul");

    // Assert new token is correctly added
    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 5); // 4 + 1 new token
    let (id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &token_id)
        .unwrap();
    let (_type, metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    assert_eq!(
        metadata,
        &BridgeTokenMetadata {
            id: *id,
            decimal_multiplier: 1_000_000_000,
            notional_value: token_price,
            native_token: false,
        }
    );

    // Add new token on EVM
    let config_address = bridge_test_cluster.contracts().bridge_config;
    let eth_signer = bridge_test_cluster.get_eth_signer();
    let eth_call = build_eth_transaction(config_address, certified_eth_action)
        .await
        .unwrap();
    let eth_receipt = send_eth_tx_and_get_tx_receipt(eth_signer.clone(), eth_call).await;
    assert!(eth_receipt.status());

    // Verify new tokens are added on EVM
    let (address, dp, price) = bridge_test_cluster
        .eth_env()
        .get_supported_token(token_id)
        .await;
    assert_eq!(address, new_token_erc_address);
    assert_eq!(dp, 9);
    assert_eq!(price, token_price);

    initiate_bridge_erc20_to_haneul(
        &bridge_test_cluster,
        100,
        new_token_erc_address,
        token_id,
        0,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_committee_registration() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_keys = vec![];
    for _ in 0..=3 {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        bridge_keys.push(kp);
    }
    let test_cluster = TestClusterWrapperBuilder::new()
        .with_bridge_authority_keys(bridge_keys)
        .build()
        .await;

    let bridge = get_bridge(
        test_cluster
            .inner
            .fullnode_handle
            .haneul_node
            .state()
            .get_object_store(),
    )
    .unwrap();

    // Member should be empty before end of epoch
    assert!(bridge.committee().members.contents.is_empty());
    assert_eq!(
        test_cluster.inner.swarm.active_validators().count(),
        bridge.committee().member_registrations.contents.len()
    );

    test_cluster
        .trigger_reconfiguration_if_not_yet_and_assert_bridge_committee_initialized()
        .await;
}
