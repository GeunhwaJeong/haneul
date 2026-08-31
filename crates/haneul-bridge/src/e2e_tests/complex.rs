// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::abi::{EthHaneulBridge, eth_haneul_bridge};
use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::e2e_tests::test_utils::{
    BridgeTestClusterBuilder, get_signatures, initiate_bridge_eth_to_haneul,
    initiate_bridge_haneul_to_eth, initiate_bridge_haneul_to_eth_v2,
    send_eth_tx_and_get_tx_receipt, wait_for_eth_coin_owned_by,
};
use crate::haneul_transaction_builder::build_haneul_transaction;
use crate::types::{BridgeAction, BridgeActionStatus, EmergencyAction, EmergencyActionType};
use alloy::primitives::{Address as EthAddress, U256};
use haneul_types::bridge::{BridgeChainId, TOKEN_ID_ETH};
use haneul_types::coin::Coin;
use haneul_types::effects::TransactionEffectsAPI;
use std::sync::Arc;
use tracing::info;

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_haneul_bridge_paused() {
    telemetry_subscribers::init_for_testing();

    // approve pause action in bridge nodes
    let pause_action = BridgeAction::EmergencyAction(EmergencyAction {
        nonce: 0,
        chain_id: BridgeChainId::HaneulCustom,
        action_type: EmergencyActionType::Pause,
    });

    let unpause_action = BridgeAction::EmergencyAction(EmergencyAction {
        nonce: 1,
        chain_id: BridgeChainId::HaneulCustom,
        action_type: EmergencyActionType::Unpause,
    });

    // Setup bridge test env
    let bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(4)
        .with_approved_governance_actions(vec![
            vec![pause_action.clone(), unpause_action.clone()],
            vec![unpause_action.clone()],
            vec![unpause_action.clone()],
            vec![],
        ])
        .build()
        .await;

    let bridge_client = bridge_test_cluster.bridge_client();
    let haneul_address = bridge_test_cluster.haneul_user_address();
    let haneul_token_type_tags = bridge_client.get_token_id_map().await.unwrap();

    // verify bridge are not paused
    assert!(!bridge_client.get_bridge_summary().await.unwrap().is_frozen);

    // try bridge from eth and verify it works on haneul
    initiate_bridge_eth_to_haneul(&bridge_test_cluster, 10, 0)
        .await
        .unwrap();
    // verify Eth was transferred to Haneul address
    let eth_coin_type = haneul_token_type_tags.get(&TOKEN_ID_ETH).unwrap();
    let eth_coin = bridge_test_cluster
        .test_cluster
        .inner
        .grpc_client()
        .get_owned_objects(
            haneul_address,
            Some(Coin::type_(eth_coin_type.clone())),
            None,
            None,
        )
        .await
        .unwrap()
        .items;
    assert_eq!(1, eth_coin.len());

    // get pause bridge signatures from committee
    let bridge_committee = Arc::new(bridge_client.get_bridge_committee().await.unwrap());
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action = agg
        .request_committee_signatures(pause_action)
        .await
        .unwrap();

    // execute pause bridge on haneul
    let gas = bridge_test_cluster
        .wallet()
        .get_one_gas_object_owned_by_address(haneul_address)
        .await
        .unwrap()
        .unwrap();

    let tx = build_haneul_transaction(
        haneul_address,
        &gas,
        certified_action,
        bridge_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await,
        &haneul_token_type_tags,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    assert!(response.effects.status().is_ok(),);
    info!("Bridge paused");

    // verify bridge paused
    assert!(bridge_client.get_bridge_summary().await.unwrap().is_frozen);

    // Transfer from eth to haneul should fail on Haneul
    let eth_to_haneul_bridge_action =
        initiate_bridge_eth_to_haneul(&bridge_test_cluster, 10, 1).await;
    assert!(eth_to_haneul_bridge_action.is_err());
    // message should not be recorded on Haneul when the bridge is paused
    let res = bridge_test_cluster
        .bridge_client()
        .get_token_transfer_action_onchain_status_until_success(
            bridge_test_cluster.eth_chain_id() as u8,
            1,
        )
        .await;
    assert_eq!(BridgeActionStatus::NotFound, res);
    // Transfer from Haneul to eth should fail
    let haneul_to_eth_bridge_action = initiate_bridge_haneul_to_eth(
        &bridge_test_cluster,
        EthAddress::random(),
        eth_coin.first().unwrap().compute_object_reference(),
        0,
        10,
    )
    .await;
    assert!(haneul_to_eth_bridge_action.is_err())
}

/// Tests the scenario where bridge nodes and Haneul framework are upgraded to V2,
/// but EVM contracts remain on V1.
///
/// Expected behavior:
/// - V1 ETH→Haneul: should work (V1 deposit on EVM, V1 claim on Haneul)
/// - V1 Haneul→ETH: should work (V1 deposit on Haneul, V1 claim on EVM)
/// - V2 Haneul→ETH: deposit and approval on Haneul succeed, but claiming on EVM fails
///   because V1 EVM contract doesn't have `transferBridgedTokensWithSignaturesV2`
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_v2_haneul_with_v1_evm() {
    telemetry_subscribers::init_for_testing();

    let haneul_chain_id = BridgeChainId::HaneulCustom as u8;
    let timer = std::time::Instant::now();
    let bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );

    // NOTE: We intentionally do NOT call upgrade_bridge_to_v2() here.
    // EVM stays on V1, while Haneul framework already has V2 functions available.

    let (eth_signer, _) = bridge_test_cluster.get_eth_signer_and_address().unwrap();
    let haneul_address = bridge_test_cluster.haneul_user_address();

    // === Test 1: V1 ETH→Haneul should work ===
    let timer = std::time::Instant::now();
    let amount = 15;
    let haneul_amount = amount * 100_000_000;

    initiate_bridge_eth_to_haneul(&bridge_test_cluster, amount, 0)
        .await
        .unwrap();
    info!(
        "[Timer] V1 Eth to Haneul bridge transfer finished in {:?}",
        timer.elapsed()
    );

    // Verify ETH was received on Haneul
    let eth_coin = wait_for_eth_coin_owned_by(&bridge_test_cluster, haneul_address, None).await;
    let (_ty, balance) = Coin::extract_balance_if_coin(&eth_coin).unwrap().unwrap();
    assert_eq!(balance, haneul_amount);

    // === Test 2: V1 Haneul→ETH should work ===
    let timer = std::time::Instant::now();
    let eth_address_1 = EthAddress::random();

    let haneul_to_eth_bridge_action = initiate_bridge_haneul_to_eth(
        &bridge_test_cluster,
        eth_address_1,
        eth_coin.compute_object_reference(),
        0, // nonce
        haneul_amount,
    )
    .await
    .unwrap();
    info!(
        "[Timer] V1 Haneul to Eth bridge transfer approved in {:?}",
        timer.elapsed()
    );

    // Claim on EVM using V1 function
    let message: eth_haneul_bridge::BridgeUtils::Message =
        haneul_to_eth_bridge_action.try_into().unwrap();
    let signatures = get_signatures(bridge_test_cluster.bridge_client(), 0, haneul_chain_id).await;
    let eth_haneul_bridge = EthHaneulBridge::new(
        bridge_test_cluster.contracts().haneul_bridge,
        eth_signer.clone(),
    );
    let call = eth_haneul_bridge.transferBridgedTokensWithSignatures(signatures, message);
    let eth_claim_tx_receipt =
        send_eth_tx_and_get_tx_receipt(eth_signer, call.into_transaction_request()).await;
    assert!(eth_claim_tx_receipt.status());
    info!(
        "[Timer] V1 Haneul to Eth bridge transfer claimed in {:?}",
        timer.elapsed()
    );

    // === Test 3: V2 Haneul→ETH deposit + approval should work, but EVM claim should fail ===
    // First, do another ETH→Haneul to get coins for V2 test
    let timer = std::time::Instant::now();
    initiate_bridge_eth_to_haneul(&bridge_test_cluster, amount, 1)
        .await
        .unwrap();
    info!(
        "[Timer] Second Eth to Haneul transfer finished in {:?}",
        timer.elapsed()
    );

    // Exclude the coin from Test 1: it was bridged away in Test 2, but a lagging index may
    // still list it.
    let eth_coin_for_v2 =
        wait_for_eth_coin_owned_by(&bridge_test_cluster, haneul_address, Some(eth_coin.id())).await;

    // Initiate V2 Haneul→ETH deposit (this should work on Haneul side)
    let timer = std::time::Instant::now();
    let eth_address_2 = EthAddress::random();

    let haneul_to_eth_v2_action = initiate_bridge_haneul_to_eth_v2(
        &bridge_test_cluster,
        eth_address_2,
        eth_coin_for_v2.compute_object_reference(),
        1, // nonce
        haneul_amount,
    )
    .await
    .unwrap();
    info!(
        "[Timer] V2 Haneul to Eth bridge transfer approved in {:?} (Haneul side)",
        timer.elapsed()
    );

    // Now try to claim on EVM using V2 function - this should fail because EVM is still on V1
    let message_v2: eth_haneul_bridge::BridgeUtils::Message =
        haneul_to_eth_v2_action.try_into().unwrap();
    let signatures_v2 =
        get_signatures(bridge_test_cluster.bridge_client(), 1, haneul_chain_id).await;

    // The V1 EVM contract doesn't have transferBridgedTokensWithSignaturesV2,
    // so calling it will fail. We verify this by attempting the call.
    let call_v2 =
        eth_haneul_bridge.transferBridgedTokensWithSignaturesV2(signatures_v2, message_v2);
    let result = call_v2.send().await;

    // The call should fail since V1 contract doesn't have this function
    assert!(
        result.is_err(),
        "V2 claim on V1 EVM contract should fail, but succeeded"
    );
    info!("V2 claim on V1 EVM correctly failed as expected");
}

/// Tests that a V1 deposit initiated before a V2 upgrade can still be claimed
/// after the upgrade completes.
///
/// This simulates the scenario where:
/// 1. User initiates a V1 ETH→Haneul deposit
/// 2. Bridge upgrades to V2 while the deposit is in flight
/// 3. The V1 deposit should still be claimable on Haneul (V2 is backwards compatible)
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_v1_deposit_during_v2_upgrade() {
    telemetry_subscribers::init_for_testing();

    let timer = std::time::Instant::now();
    // Start without bridge cluster - we'll start it manually to control timing
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster (without nodes) started in {:?}",
        timer.elapsed()
    );

    let haneul_address = bridge_test_cluster.haneul_user_address();
    let (eth_signer, _) = bridge_test_cluster.get_eth_signer_and_address().unwrap();

    // === Step 1: Initiate V1 ETH→Haneul deposit on EVM BEFORE bridge cluster starts ===
    let timer = std::time::Instant::now();
    let amount = 20;
    let haneul_amount = amount * 100_000_000;

    // Deposit ETH to EVM contract (V1 deposit)
    let contract = EthHaneulBridge::new(
        bridge_test_cluster.contracts().haneul_bridge,
        eth_signer.clone(),
    );
    let haneul_recipient = haneul_address.to_vec();
    let deposit_call = contract
        .bridgeETH(haneul_recipient.into(), BridgeChainId::HaneulCustom as u8)
        .value(U256::from(amount) * U256::from(10).pow(U256::from(18)));
    let tx_receipt =
        send_eth_tx_and_get_tx_receipt(eth_signer.clone(), deposit_call.into_transaction_request())
            .await;
    assert!(tx_receipt.status());
    info!(
        "[Timer] V1 ETH deposit on EVM completed in {:?}",
        timer.elapsed()
    );

    // === Step 2: Upgrade EVM to V2 BEFORE starting bridge cluster ===
    let timer = std::time::Instant::now();
    bridge_test_cluster
        .upgrade_bridge_to_v2()
        .await
        .expect("Failed to upgrade bridge to V2");
    info!("[Timer] Bridge upgraded to V2 in {:?}", timer.elapsed());

    // === Step 3: Now start the bridge cluster ===
    // The bridge nodes will see the V1 deposit event and should still process it
    let timer = std::time::Instant::now();
    // Must set governance actions for the correct number of validators (3)
    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![],
        vec![],
        vec![],
    ]);
    bridge_test_cluster.start_bridge_cluster().await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("[Timer] Bridge cluster started in {:?}", timer.elapsed());

    // === Step 4: Wait for the V1 deposit to be claimed on Haneul ===
    // Even though bridge is now V2, the V1 message should still be processable
    let timer = std::time::Instant::now();
    let now = std::time::Instant::now();
    loop {
        let res = bridge_test_cluster
            .bridge_client()
            .get_token_transfer_action_onchain_status_until_success(
                bridge_test_cluster.eth_chain_id() as u8,
                0, // nonce
            )
            .await;
        if res == BridgeActionStatus::Claimed {
            info!(
                "[Timer] V1 deposit claimed after V2 upgrade in {:?}",
                timer.elapsed()
            );
            break;
        }
        if now.elapsed().as_secs() > 120 {
            panic!(
                "Timeout waiting for V1 deposit to be claimed after V2 upgrade. Status: {:?}",
                res
            );
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    // Verify the ETH coin was received on Haneul
    let eth_coin = wait_for_eth_coin_owned_by(&bridge_test_cluster, haneul_address, None).await;
    let (_ty, balance) = Coin::extract_balance_if_coin(&eth_coin).unwrap().unwrap();
    assert_eq!(balance, haneul_amount);
    info!("V1 deposit successfully claimed after V2 upgrade - backwards compatibility confirmed!");

    // === Optional: Verify V2 operations still work after upgrade ===
    // Now that we're fully on V2, test a V2 Haneul→ETH transfer
    let timer = std::time::Instant::now();
    let eth_address = EthAddress::random();
    let haneul_to_eth_v2_action = initiate_bridge_haneul_to_eth_v2(
        &bridge_test_cluster,
        eth_address,
        eth_coin.compute_object_reference(),
        0, // nonce for Haneul→ETH direction
        haneul_amount,
    )
    .await
    .unwrap();
    info!(
        "[Timer] V2 Haneul to Eth transfer approved in {:?}",
        timer.elapsed()
    );

    // Claim on EVM using V2 function (should work now that EVM is upgraded)
    let timer = std::time::Instant::now();
    let message: eth_haneul_bridge::BridgeUtils::Message =
        haneul_to_eth_v2_action.try_into().unwrap();
    let signatures = get_signatures(
        bridge_test_cluster.bridge_client(),
        0,
        BridgeChainId::HaneulCustom as u8,
    )
    .await;
    // Get a fresh eth_signer to avoid nonce issues after upgrade
    let (eth_signer, _) = bridge_test_cluster.get_eth_signer_and_address().unwrap();
    let eth_haneul_bridge = EthHaneulBridge::new(
        bridge_test_cluster.contracts().haneul_bridge,
        eth_signer.clone(),
    );
    let call = eth_haneul_bridge.transferBridgedTokensWithSignaturesV2(signatures, message);
    let eth_claim_tx_receipt =
        send_eth_tx_and_get_tx_receipt(eth_signer, call.into_transaction_request()).await;
    assert!(eth_claim_tx_receipt.status());
    info!(
        "[Timer] V2 Haneul to Eth transfer claimed in {:?}",
        timer.elapsed()
    );
    info!("V2 operations work correctly after upgrade!");
}
