// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#![allow(deprecated)]

use haneul_json::{call_args, type_args};
use haneul_json_rpc_api::{
    CoinReadApiClient, GovernanceReadApiClient, IndexerApiClient, ReadApiClient,
    TransactionBuilderClient, WriteApiClient,
};
use haneul_json_rpc_types::ObjectsPage;
use haneul_json_rpc_types::{
    Balance, CoinPage, DelegatedStake, HaneulCoinMetadata, HaneulExecutionStatus,
    HaneulObjectDataOptions, HaneulObjectResponse, HaneulObjectResponseQuery,
    HaneulTransactionBlockEffectsAPI, HaneulTransactionBlockResponse,
    HaneulTransactionBlockResponseOptions, StakeStatus, TransactionBlockBytes,
};
use haneul_json_rpc_types::{ObjectChange, ZkLoginIntentScope};
use haneul_macros::sim_test;
use haneul_move_build::BuildConfig;
use haneul_simulator::fastcrypto::encoding::{Base64, Encoding};
use haneul_swarm_config::genesis_config::{
    DEFAULT_GAS_AMOUNT, DEFAULT_NUMBER_OF_OBJECT_PER_ACCOUNT,
};
use haneul_test_transaction_builder::TestTransactionBuilder;
use haneul_test_transaction_builder::make_transfer_haneul_transaction;
use haneul_types::balance::Supply;
use haneul_types::base_types::SequenceNumber;
use haneul_types::base_types::{HaneulAddress, ObjectID};
use haneul_types::coin::{COIN_MODULE_NAME, TreasuryCap};
use haneul_types::crypto::Signature;
use haneul_types::digests::ObjectDigest;
use haneul_types::gas_coin::GAS;
use haneul_types::signature::GenericSignature;
use haneul_types::transaction_driver_types::ExecuteTransactionRequestType;
use haneul_types::utils::load_test_vectors;
use haneul_types::zk_login_authenticator::ZkLoginAuthenticator;
use haneul_types::{
    HANEUL_DISPLAY_REGISTRY_OBJECT_ID, HANEUL_FRAMEWORK_ADDRESS, parse_haneul_struct_tag,
};
use serde_json::json;
use shared_crypto::intent::{Intent, IntentMessage};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
#[cfg(not(msim))]
use std::str::FromStr;
use std::time::Duration;
use test_cluster::TestClusterBuilder;
use tokio::time::sleep;

#[sim_test]
async fn test_get_objects() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;

    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new(),
            )),
            None,
            None,
        )
        .await?;
    assert_eq!(5, objects.data.len());

    // Multiget objectIDs test
    let object_digests = objects
        .data
        .iter()
        .map(|o| o.object().unwrap().object_id)
        .collect();

    let object_resp = http_client.multi_get_objects(object_digests, None).await?;
    assert_eq!(5, object_resp.len());
    Ok(())
}

#[tokio::test]
async fn test_get_package_with_display_should_not_fail() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let response = http_client
        .get_object(
            ObjectID::from(HANEUL_FRAMEWORK_ADDRESS),
            Some(HaneulObjectDataOptions::new().with_display()),
        )
        .await;
    assert!(response.is_ok());
    let response: HaneulObjectResponse = response?;
    assert!(
        response
            .into_object()
            .unwrap()
            .display
            .unwrap()
            .data
            .is_none()
    );
    Ok(())
}

#[sim_test]
async fn test_get_object_with_display_v2_should_render() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let gas = objects.first().unwrap().object().unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "data", "display_v2"]);
    let compiled_package = BuildConfig::new_for_testing().build_async(&path).await?;
    let compiled_modules_bytes =
        compiled_package.get_package_base64(/* with_unpublished_deps */ false);
    let dependencies = compiled_package.get_dependency_storage_package_ids();

    let transaction_bytes: TransactionBlockBytes = http_client
        .publish(
            address,
            compiled_modules_bytes,
            dependencies,
            Some(gas.object_id),
            100_000_000.into(),
        )
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(HaneulTransactionBlockResponseOptions::new().with_object_changes()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let object_changes = tx_response.object_changes.unwrap();
    let package_id = object_changes
        .iter()
        .find_map(|e| {
            if let ObjectChange::Published { package_id, .. } = e {
                Some(*package_id)
            } else {
                None
            }
        })
        .unwrap();

    let setup_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            "display_v2".to_string(),
            "setup_display".to_string(),
            type_args![]?,
            call_args!(HANEUL_DISPLAY_REGISTRY_OBJECT_ID)?,
            Some(gas.object_id),
            50_000_000.into(),
            None,
        )
        .await?;

    let setup_tx = cluster
        .wallet
        .sign_transaction(&setup_bytes.to_data()?)
        .await;
    let (setup_tx_bytes, setup_signatures) = setup_tx.to_tx_bytes_and_signatures();

    let setup_response = http_client
        .execute_transaction_block(
            setup_tx_bytes,
            setup_signatures,
            Some(HaneulTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    assert_eq!(
        HaneulExecutionStatus::Success,
        *setup_response.effects.unwrap().status()
    );

    let mint_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            "display_v2".to_string(),
            "mint".to_string(),
            type_args![]?,
            call_args!(42u64, "hello", address, address)?,
            Some(gas.object_id),
            10_000_000.into(),
            None,
        )
        .await?;

    let mint_tx = cluster
        .wallet
        .sign_transaction(&mint_bytes.to_data()?)
        .await;
    let (mint_tx_bytes, mint_signatures) = mint_tx.to_tx_bytes_and_signatures();

    let mint_response = http_client
        .execute_transaction_block(
            mint_tx_bytes,
            mint_signatures,
            Some(HaneulTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let created = mint_response.effects.unwrap().created().to_vec();
    let created_id = created.first().unwrap().reference.object_id;

    let add_df_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            "display_v2".to_string(),
            "add_df".to_string(),
            type_args![]?,
            call_args!(created_id)?,
            Some(gas.object_id),
            10_000_000.into(),
            None,
        )
        .await?;

    let add_df_tx = cluster
        .wallet
        .sign_transaction(&add_df_bytes.to_data()?)
        .await;
    let (add_df_tx_bytes, add_df_signatures) = add_df_tx.to_tx_bytes_and_signatures();

    http_client
        .execute_transaction_block(
            add_df_tx_bytes,
            add_df_signatures,
            Some(HaneulTransactionBlockResponseOptions::new()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let response = http_client
        .get_object(
            created_id,
            Some(HaneulObjectDataOptions::new().with_display().with_type()),
        )
        .await?;
    let object = response.into_object()?;
    let display = object.display.unwrap();
    let display_data = display.data.unwrap();

    assert!(display.error.is_none());
    let expected_display = json!({
        "bar": "bar is 42!",
        "baz": "baz is true?",
        "qux": {
            "val": "hello",
            "quy": {
                "val": format!("{address}"),
                "quz": {
                    "val": 43
                }
            }
        },
        "quy": format!("quy is {address}."),
        "qu_": format!("x(hello) y({address}), z(43)?!"),
        "f42": "[42] is 42004200"
    });
    assert_eq!(serde_json::to_value(display_data)?, expected_display);

    Ok(())
}

#[sim_test]
async fn test_public_transfer_object() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let obj = objects.clone().first().unwrap().object().unwrap().object_id;
    let gas = objects.clone().last().unwrap().object().unwrap().object_id;

    let transaction_bytes: TransactionBlockBytes = http_client
        .transfer_object(address, obj, Some(gas), 1_000_000.into(), address)
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
    let tx_bytes1 = tx_bytes.clone();
    let dryrun_response = http_client.dry_run_transaction_block(tx_bytes).await?;

    let tx_response: HaneulTransactionBlockResponse = http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures,
            Some(
                HaneulTransactionBlockResponseOptions::new()
                    .with_effects()
                    .with_object_changes(),
            ),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    assert_same_object_changes_ignoring_version_and_digest(
        dryrun_response.object_changes,
        tx_response.object_changes.unwrap(),
    );
    Ok(())
}

fn assert_same_object_changes_ignoring_version_and_digest(
    expected: Vec<ObjectChange>,
    actual: Vec<ObjectChange>,
) {
    fn collect_changes_mask_version_and_digest(
        changes: Vec<ObjectChange>,
    ) -> BTreeMap<ObjectID, ObjectChange> {
        changes
            .into_iter()
            .map(|mut change| {
                let object_id = change.object_id();
                // ignore the version and digest for comparison
                change.mask_for_test(SequenceNumber::MAX, ObjectDigest::MAX);
                (object_id, change)
            })
            .collect()
    }
    let expected = collect_changes_mask_version_and_digest(expected);
    let actual = collect_changes_mask_version_and_digest(actual);
    assert!(expected.keys().all(|id| actual.contains_key(id)));
    assert!(actual.keys().all(|id| expected.contains_key(id)));
    for (id, exp) in &expected {
        let act = actual.get(id).unwrap();
        assert_eq!(act, exp);
    }
}

#[sim_test]
async fn test_publish() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?;
    let gas = objects.data.first().unwrap().object().unwrap();

    let compiled_package = BuildConfig::new_for_testing()
        .build_async(Path::new("../../examples/move/basics"))
        .await?;
    let compiled_modules_bytes =
        compiled_package.get_package_base64(/* with_unpublished_deps */ false);
    let dependencies = compiled_package.get_dependency_storage_package_ids();

    let transaction_bytes: TransactionBlockBytes = http_client
        .publish(
            address,
            compiled_modules_bytes,
            dependencies,
            Some(gas.object_id),
            100_000_000.into(),
        )
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(HaneulTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    matches!(tx_response, HaneulTransactionBlockResponse {effects, ..} if effects.as_ref().unwrap().created().len() == 6);
    Ok(())
}

#[sim_test]
async fn test_move_call() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas = objects.first().unwrap().object().unwrap();
    let coin = &objects[1].object()?;

    // now do the call
    let package_id = ObjectID::new(HANEUL_FRAMEWORK_ADDRESS.into_bytes());
    let module = "pay".to_string();
    let function = "split".to_string();

    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            type_args![GAS::type_tag()]?,
            call_args!(coin.object_id, 10)?,
            Some(gas.object_id),
            10_000_000.into(),
            None,
        )
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;

    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(HaneulTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    matches!(tx_response, HaneulTransactionBlockResponse {effects, ..} if effects.as_ref().unwrap().created().len() == 1);
    Ok(())
}

#[sim_test]
async fn test_get_object_info() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    for obj in objects {
        let oref = obj.into_object().unwrap();
        let result = http_client
            .get_object(
                oref.object_id,
                Some(HaneulObjectDataOptions::new().with_owner()),
            )
            .await?;
        assert!(
            matches!(result, HaneulObjectResponse { data: Some(object), .. } if oref.object_id == object.object_id && object.owner.clone().unwrap().get_owner_address()? == address)
        );
    }
    Ok(())
}

#[sim_test]
async fn test_get_object_data_with_content() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new().with_content().with_owner(),
            )),
            None,
            None,
        )
        .await?
        .data;

    for obj in objects {
        let oref = obj.into_object().unwrap();
        let result = http_client
            .get_object(
                oref.object_id,
                Some(HaneulObjectDataOptions::new().with_content().with_owner()),
            )
            .await?;
        assert!(
            matches!(result, HaneulObjectResponse { data: Some(object), .. } if oref.object_id == object.object_id && object.owner.clone().unwrap().get_owner_address()? == address)
        );
    }
    Ok(())
}

#[sim_test]
async fn test_get_coins() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let result: CoinPage = http_client.get_coins(address, None, None, None).await?;
    assert_eq!(5, result.data.len());
    assert!(!result.has_next_page);

    let result: CoinPage = http_client
        .get_coins(address, Some("0x2::haneul::TestCoin".into()), None, None)
        .await?;
    assert_eq!(0, result.data.len());

    let result: CoinPage = http_client
        .get_coins(address, Some("0x2::haneul::HANEUL".into()), None, None)
        .await?;
    assert_eq!(5, result.data.len());
    assert!(!result.has_next_page);

    // Test paging
    let result: CoinPage = http_client
        .get_coins(address, Some("0x2::haneul::HANEUL".into()), None, Some(3))
        .await?;
    assert_eq!(3, result.data.len());
    assert!(result.has_next_page);

    let result: CoinPage = http_client
        .get_coins(
            address,
            Some("0x2::haneul::HANEUL".into()),
            result.next_cursor,
            Some(3),
        )
        .await?;
    assert_eq!(2, result.data.len(), "{:?}", result);
    assert!(!result.has_next_page);

    Ok(())
}

#[sim_test]
async fn test_sorted_get_coin_response() {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();

    let address = HaneulAddress::random_for_testing_only();

    // send 5 coins to address `address` with different values
    let amounts = [1, 2, 3, 4, 5];
    for amount in amounts {
        let tx =
            make_transfer_haneul_transaction(&cluster.wallet, Some(address), Some(amount)).await;

        cluster.execute_transaction(tx).await;
    }

    let coins: CoinPage = http_client
        .get_coins(address, None, None, None)
        .await
        .unwrap();
    assert_eq!(amounts.len(), coins.data.len());

    let balances = coins
        .data
        .iter()
        .map(|coin| coin.balance)
        .collect::<Vec<_>>();
    let mut sorted_amounts = amounts;
    sorted_amounts.reverse();
    assert_eq!(sorted_amounts.as_slice(), balances.as_slice());
}

#[sim_test]
async fn test_get_balance() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let result: Balance = http_client.get_balance(address, None).await?;
    assert_eq!("0x2::haneul::HANEUL", result.coin_type);
    assert_eq!(
        (DEFAULT_NUMBER_OF_OBJECT_PER_ACCOUNT as u64 * DEFAULT_GAS_AMOUNT) as u128,
        result.total_balance
    );
    assert_eq!(
        DEFAULT_NUMBER_OF_OBJECT_PER_ACCOUNT,
        result.coin_object_count
    );
    Ok(())
}

#[sim_test]
async fn test_get_metadata() -> Result<(), anyhow::Error> {
    telemetry_subscribers::init_for_testing();

    let cluster = TestClusterBuilder::new().build().await;

    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;

    let gas = objects.first().unwrap().object().unwrap();

    // Publish test coin package
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "data", "dummy_modules_publish"]);
    let compiled_package = BuildConfig::new_for_testing().build_async(&path).await?;
    let compiled_modules_bytes =
        compiled_package.get_package_base64(/* with_unpublished_deps */ false);
    let dependencies = compiled_package.get_dependency_storage_package_ids();

    let transaction_bytes: TransactionBlockBytes = http_client
        .publish(
            address,
            compiled_modules_bytes,
            dependencies,
            Some(gas.object_id),
            100_000_000.into(),
        )
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(
                HaneulTransactionBlockResponseOptions::new()
                    .with_object_changes()
                    .with_events(),
            ),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let object_changes = tx_response.object_changes.unwrap();
    let package_id = object_changes
        .iter()
        .find_map(|e| {
            if let ObjectChange::Published { package_id, .. } = e {
                Some(package_id)
            } else {
                None
            }
        })
        .unwrap();

    let result: HaneulCoinMetadata = http_client
        .get_coin_metadata(format!("{package_id}::trusted_coin::TRUSTED_COIN"))
        .await?
        .unwrap();

    assert_eq!("TRUSTED", result.symbol);
    assert_eq!("Trusted Coin for test", result.description);
    assert_eq!("Trusted Coin", result.name);
    assert_eq!(2, result.decimals);

    Ok(())
}

#[sim_test]
async fn test_get_total_supply() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;

    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let gas = objects.first().unwrap().object().unwrap();

    // Publish test coin package
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "data", "dummy_modules_publish"]);
    let compiled_package = BuildConfig::new_for_testing().build_async(&path).await?;
    let compiled_modules_bytes =
        compiled_package.get_package_base64(/* with_unpublished_deps */ false);
    let dependencies = compiled_package.get_dependency_storage_package_ids();

    let transaction_bytes: TransactionBlockBytes = http_client
        .publish(
            address,
            compiled_modules_bytes,
            dependencies,
            Some(gas.object_id),
            100_000_000.into(),
        )
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let tx_response: HaneulTransactionBlockResponse = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(
                HaneulTransactionBlockResponseOptions::new()
                    .with_object_changes()
                    .with_events(),
            ),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let object_changes = tx_response.object_changes.as_ref().unwrap();
    let package_id = object_changes
        .iter()
        .find_map(|e| {
            if let ObjectChange::Published { package_id, .. } = e {
                Some(package_id)
            } else {
                None
            }
        })
        .unwrap();

    let coin_name = format!("{package_id}::trusted_coin::TRUSTED_COIN");
    let result: Supply = http_client.get_total_supply(coin_name.clone()).await?;

    assert_eq!(0, result.value);

    let object_changes = tx_response.object_changes.as_ref().unwrap();
    let treasury_cap = object_changes
        .iter()
        .find_map(|e| {
            if let ObjectChange::Created {
                object_id,
                object_type,
                ..
            } = e
            {
                if &TreasuryCap::type_(parse_haneul_struct_tag(&coin_name).unwrap()) == object_type
                {
                    Some(object_id)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    // Mint 100000 coin

    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            HANEUL_FRAMEWORK_ADDRESS.into(),
            COIN_MODULE_NAME.to_string(),
            "mint_and_transfer".into(),
            type_args![coin_name]?,
            call_args![treasury_cap, 100000, address]?,
            Some(gas.object_id),
            10_000_000.into(),
            None,
        )
        .await?;

    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(HaneulTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let HaneulTransactionBlockResponse { effects, .. } = tx_response;

    assert_eq!(HaneulExecutionStatus::Success, *effects.unwrap().status());

    let result: Supply = http_client.get_total_supply(coin_name.clone()).await?;
    assert_eq!(100000, result.value);

    Ok(())
}

#[sim_test]
async fn test_staking() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;

    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects: ObjectsPage = http_client
        .get_owned_objects(
            address,
            Some(HaneulObjectResponseQuery::new_with_options(
                HaneulObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?;
    assert_eq!(5, objects.data.len());

    // Check StakedHaneul object before test
    let staked_haneul: Vec<DelegatedStake> = http_client.get_stakes(address).await?;
    assert!(staked_haneul.is_empty());

    let validator = http_client
        .get_latest_haneul_system_state()
        .await?
        .active_validators[0]
        .haneul_address;

    let coin = objects.data[0].object()?.object_id;
    // Delegate some HANEUL
    let transaction_bytes: TransactionBlockBytes = http_client
        .request_add_stake(
            address,
            vec![coin],
            Some(1000000000.into()),
            validator,
            None,
            100_000_000.into(),
        )
        .await?;
    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;

    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(HaneulTransactionBlockResponseOptions::new()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    // Check DelegatedStake object
    let staked_haneul: Vec<DelegatedStake> = http_client.get_stakes(address).await?;
    assert_eq!(1, staked_haneul.len());
    assert_eq!(1000000000, staked_haneul[0].stakes[0].principal);
    assert!(matches!(
        staked_haneul[0].stakes[0].status,
        StakeStatus::Pending
    ));
    let staked_haneul_copy = http_client
        .get_stakes_by_ids(vec![staked_haneul[0].stakes[0].staked_haneul_id])
        .await?;
    assert_eq!(
        staked_haneul[0].stakes[0].staked_haneul_id,
        staked_haneul_copy[0].stakes[0].staked_haneul_id
    );
    Ok(())
}

#[ignore]
#[sim_test]
async fn test_unstaking() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(10000)
        .build()
        .await;

    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let coins: CoinPage = http_client.get_coins(address, None, None, None).await?;
    assert_eq!(5, coins.data.len());

    // Check StakedHaneul object before test
    let staked_haneul: Vec<DelegatedStake> = http_client.get_stakes(address).await?;
    assert!(staked_haneul.is_empty());

    let validator = http_client
        .get_latest_haneul_system_state()
        .await?
        .active_validators[0]
        .haneul_address;

    // Delegate some HANEUL
    for i in 0..3 {
        let transaction_bytes: TransactionBlockBytes = http_client
            .request_add_stake(
                address,
                vec![coins.data[i].coin_object_id],
                Some(1000000000.into()),
                validator,
                None,
                100_000_000.into(),
            )
            .await?;
        let tx = cluster
            .wallet
            .sign_transaction(&transaction_bytes.to_data()?)
            .await;

        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

        http_client
            .execute_transaction_block(
                tx_bytes,
                signatures,
                Some(HaneulTransactionBlockResponseOptions::new()),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await?;
    }
    // Check DelegatedStake object
    let staked_haneul: Vec<DelegatedStake> = http_client.get_stakes(address).await?;
    assert_eq!(1, staked_haneul.len());
    assert_eq!(1000000000, staked_haneul[0].stakes[0].principal);

    sleep(Duration::from_millis(10000)).await;

    let staked_haneul_copy = http_client
        .get_stakes_by_ids(vec![
            staked_haneul[0].stakes[0].staked_haneul_id,
            staked_haneul[0].stakes[1].staked_haneul_id,
            staked_haneul[0].stakes[2].staked_haneul_id,
        ])
        .await?;

    assert!(matches!(
        &staked_haneul_copy[0].stakes[0].status,
        StakeStatus::Active {
            estimated_reward: _
        }
    ));
    assert!(matches!(
        &staked_haneul_copy[0].stakes[1].status,
        StakeStatus::Active {
            estimated_reward: _
        }
    ));
    assert!(matches!(
        &staked_haneul_copy[0].stakes[2].status,
        StakeStatus::Active {
            estimated_reward: _
        }
    ));

    let transaction_bytes: TransactionBlockBytes = http_client
        .request_withdraw_stake(
            address,
            staked_haneul_copy[0].stakes[2].staked_haneul_id,
            None,
            1_000_000.into(),
        )
        .await?;
    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;

    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(HaneulTransactionBlockResponseOptions::new()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    sleep(Duration::from_millis(20000)).await;

    let staked_haneul_copy = http_client
        .get_stakes_by_ids(vec![
            staked_haneul[0].stakes[0].staked_haneul_id,
            staked_haneul[0].stakes[1].staked_haneul_id,
            staked_haneul[0].stakes[2].staked_haneul_id,
        ])
        .await?;

    assert!(matches!(
        &staked_haneul_copy[0].stakes[0].status,
        StakeStatus::Active {
            estimated_reward: _
        }
    ));
    assert!(matches!(
        &staked_haneul_copy[0].stakes[1].status,
        StakeStatus::Active {
            estimated_reward: _
        }
    ));
    assert!(matches!(
        &staked_haneul_copy[0].stakes[2].status,
        StakeStatus::Unstaked
    ));
    Ok(())
}

#[sim_test]
async fn test_staking_multiple_coins() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;

    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let coins: CoinPage = http_client.get_coins(address, None, None, None).await?;
    assert_eq!(5, coins.data.len());

    let genesis_coin_amount = coins.data[0].balance;

    // Check StakedHaneul object before test
    let staked_haneul: Vec<DelegatedStake> = http_client.get_stakes(address).await?;
    assert!(staked_haneul.is_empty());

    let validator = http_client
        .get_latest_haneul_system_state()
        .await?
        .active_validators[0]
        .haneul_address;
    // Delegate some HANEUL
    let transaction_bytes: TransactionBlockBytes = http_client
        .request_add_stake(
            address,
            vec![
                coins.data[0].coin_object_id,
                coins.data[1].coin_object_id,
                coins.data[2].coin_object_id,
            ],
            Some(1000000000.into()),
            validator,
            None,
            100_000_000.into(),
        )
        .await?;
    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?)
        .await;

    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let dryrun_response = http_client
        .dry_run_transaction_block(tx_bytes.clone())
        .await?;

    let executed_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(
                HaneulTransactionBlockResponseOptions::new()
                    .with_balance_changes()
                    .with_input(),
            ),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    // Check coin balance changes on dry run
    assert_eq!(
        dryrun_response.balance_changes,
        executed_response.balance_changes.unwrap()
    );

    // Check that inputs for dry run match the executed transaction
    assert_eq!(
        dryrun_response.input,
        executed_response.transaction.unwrap().data
    );

    // Check DelegatedStake object
    let staked_haneul: Vec<DelegatedStake> = http_client.get_stakes(address).await?;
    assert_eq!(1, staked_haneul.len());
    assert_eq!(1000000000, staked_haneul[0].stakes[0].principal);
    assert!(matches!(
        staked_haneul[0].stakes[0].status,
        StakeStatus::Pending
    ));

    // Coins should be merged into one and returned to the sender.
    let coins: CoinPage = http_client.get_coins(address, None, None, None).await?;
    assert_eq!(3, coins.data.len());

    // Find the new coin
    let new_coin = coins
        .data
        .iter()
        .find(|coin| coin.balance > genesis_coin_amount)
        .unwrap();
    assert_eq!((genesis_coin_amount * 3) - 1000000000, new_coin.balance);

    Ok(())
}

#[sim_test]
async fn test_zklogin_verify() -> Result<(), anyhow::Error> {
    if haneul_simulator::has_mainnet_protocol_config_override() {
        return Ok(());
    }

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(15000)
        .with_default_jwks()
        .build()
        .await;
    test_cluster.wait_for_epoch(Some(1)).await;
    test_cluster.wait_for_authenticator_state_update().await;

    let http_client = test_cluster.rpc_client();

    // Construct a valid zkLogin transaction data, signature.
    let (kp, pk_zklogin, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let zklogin_addr = (pk_zklogin).into();
    let rgp = test_cluster.get_reference_gas_price().await;
    let gas = test_cluster
        .fund_address_and_return_gas(rgp, Some(20000000000), zklogin_addr)
        .await;
    let tx_data = TestTransactionBuilder::new(zklogin_addr, gas, rgp)
        .transfer_haneul(None, HaneulAddress::ZERO)
        .build();
    let msg = IntentMessage::new(Intent::haneul_transaction(), tx_data.clone());
    let eph_sig = Signature::new_secure(&msg, kp);
    let generic_sig = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        eph_sig.clone(),
    ));

    // construct all parameters for the query
    let bytes = Base64::encode(bcs::to_bytes(&tx_data).unwrap());
    let signature = Base64::encode(generic_sig.as_ref());
    let intent_scope = ZkLoginIntentScope::TransactionData;

    let res = http_client
        .verify_zklogin_signature(bytes.clone(), signature.clone(), intent_scope, zklogin_addr)
        .await?;
    assert!(res.success);
    assert!(res.errors.is_empty());

    let res = http_client
        .verify_zklogin_signature(
            bytes,
            signature,
            ZkLoginIntentScope::PersonalMessage,
            zklogin_addr,
        )
        .await?;
    assert!(!res.success);
    assert!(!res.errors.is_empty());
    Ok(())
}
