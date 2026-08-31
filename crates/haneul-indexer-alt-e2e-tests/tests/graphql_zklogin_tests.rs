// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::Context as _;
use anyhow::bail;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use haneul_indexer_alt_framework::ingestion::ClientArgs;
use haneul_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs;
use haneul_indexer_alt_graphql::config::RpcConfig as GraphQlConfig;
use haneul_indexer_alt_graphql::config::ZkLoginConfig;
use haneul_indexer_alt_graphql::config::ZkLoginEnv;
use haneul_swarm_config::genesis_config::AccountConfig;
use haneul_test_transaction_builder::TestTransactionBuilder;
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::Signature;
use haneul_types::signature::GenericSignature;
use haneul_types::utils::load_test_vectors;
use haneul_types::zk_login_authenticator::ZkLoginAuthenticator;
use insta::assert_debug_snapshot;
use prometheus::Registry;
use serde::Deserialize;
use serde_json::json;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentMessage;
use shared_crypto::intent::PersonalMessage;
use tempfile::TempDir;
use test_cluster::TestCluster;
use test_cluster::TestClusterBuilder;
use tokio::time::interval;

use haneul_indexer_alt_e2e_tests::OffchainCluster;
use haneul_indexer_alt_e2e_tests::OffchainClusterConfig;

const QUERY: &str = r#"
query ($bytes: Base64!, $signature: Base64!, $scope: ZkLoginIntentScope!, $author: HaneulAddress!) {
    verifyZkLoginSignature(bytes: $bytes, signature: $signature, intentScope: $scope, author: $author) {
        success
    }
}
"#;

struct FullCluster {
    /// Validator and fullnodes
    onchain: TestCluster,

    /// Indexers and RPCs
    offchain: OffchainCluster,

    #[allow(unused)]
    temp_dir: TempDir,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
struct ZkLoginResult {
    success: bool,
}

impl FullCluster {
    async fn new() -> anyhow::Result<Self> {
        let temp_dir = TempDir::new()?;
        let ingestion_dir = temp_dir.path().to_path_buf();

        let onchain = TestClusterBuilder::new()
            .with_num_validators(1)
            .with_data_ingestion_dir(ingestion_dir.clone())
            .with_epoch_duration_ms(300_000) // 5 minutes
            .with_accounts(vec![
                AccountConfig {
                    address: None,
                    gas_amounts: vec![1_000_000_000_000; 2],
                };
                4
            ])
            .build()
            .await;

        let offchain = OffchainCluster::new(
            ClientArgs {
                ingestion: IngestionClientArgs {
                    local_ingestion_path: Some(ingestion_dir),
                    ..Default::default()
                },
                ..Default::default()
            },
            OffchainClusterConfig {
                graphql_config: GraphQlConfig {
                    zklogin: ZkLoginConfig {
                        env: ZkLoginEnv::Test,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            &Registry::new(),
        )
        .await?;

        // Trigger an epoch change and wait until GraphQL sees Epoch 1
        onchain.trigger_reconfiguration().await;
        onchain.wait_for_authenticator_state_update().await;
        tokio::time::timeout(Duration::from_secs(30), async {
            let mut interval = interval(Duration::from_millis(200));
            loop {
                interval.tick().await;
                if matches!(offchain.latest_graphql_epoch().await, Ok(1)) {
                    break;
                }
            }
        })
        .await
        .unwrap();

        Ok(Self {
            onchain,
            offchain,
            temp_dir,
        })
    }

    async fn verify_zklogin(
        &self,
        bytes: Vec<u8>,
        signature: Vec<u8>,
        scope: &str,
        author: HaneulAddress,
    ) -> anyhow::Result<ZkLoginResult> {
        let client = reqwest::Client::new();
        let url = self.offchain.graphql_url();

        let variables = json!({
            "bytes": Base64::encode(bytes),
            "signature": Base64::encode(signature),
            "scope": scope,
            "author": author.to_string(),
        });

        let response: serde_json::Value = client
            .post(url.as_str())
            .json(&json!({
                "query": QUERY,
                "variables": variables
            }))
            .send()
            .await?
            .json()
            .await?;

        if let Some(errors) = response.get("errors").and_then(|es| es.as_array()) {
            let errors: Vec<_> = errors
                .iter()
                .map(|e| e.get("message").unwrap().as_str().unwrap().to_owned())
                .collect();

            bail!(serde_json::to_string(&errors).unwrap());
        }

        let result = response
            .pointer("/data/verifyZkLoginSignature")
            .with_context(|| format!("missing data.verifyZkLoginSignature in {response:#?}"))?;

        serde_json::from_value(result.clone()).context("failed to deserialize result")
    }
}

#[tokio::test]
async fn test_verify_transaction() {
    let cluster = FullCluster::new().await.unwrap();

    let (kp, pk, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let addr = pk.into();
    let rgp = cluster.onchain.get_reference_gas_price().await;
    let gas = cluster
        .onchain
        .fund_address_and_return_gas(rgp, Some(1000000), addr)
        .await;

    let tx = TestTransactionBuilder::new(addr, gas, rgp)
        .transfer_haneul(None, HaneulAddress::ZERO)
        .build();

    let message = IntentMessage::new(Intent::haneul_transaction(), tx.clone());
    let signature = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        Signature::new_secure(&message, kp),
    ));

    let result = cluster
        .verify_zklogin(
            bcs::to_bytes(&tx).unwrap(),
            signature.as_ref().to_owned(),
            "TRANSACTION_DATA",
            addr,
        )
        .await
        .unwrap();

    assert_eq!(result, ZkLoginResult { success: true });
}

#[tokio::test]
async fn test_verify_personal_message() {
    let cluster = FullCluster::new().await.unwrap();

    let (kp, pk, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let addr = pk.into();
    let personal = b"Hello, World!".to_vec();

    let message = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: personal.clone(),
        },
    );

    let signature = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        Signature::new_secure(&message, kp),
    ));

    let result = cluster
        .verify_zklogin(
            personal,
            signature.as_ref().to_owned(),
            "PERSONAL_MESSAGE",
            addr,
        )
        .await
        .unwrap();

    assert_eq!(result, ZkLoginResult { success: true });
}

#[tokio::test]
async fn test_verify_zklogin_payload_bypasses_query_limit() {
    let cluster = FullCluster::new().await.unwrap();

    let (kp, pk, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let addr = pk.into();
    let payload_size = GraphQlConfig::default().limits.max_query_payload_size as usize;
    let bytes = vec![0u8; payload_size + 64];

    let message = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: bytes.clone(),
        },
    );
    let signature = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        Signature::new_secure(&message, kp),
    ));

    let result = cluster
        .verify_zklogin(
            bytes,
            signature.as_ref().to_owned(),
            "PERSONAL_MESSAGE",
            addr,
        )
        .await
        .unwrap();

    assert_eq!(result, ZkLoginResult { success: true });
}

#[tokio::test]
async fn test_verify_invalid_scope() {
    let cluster = FullCluster::new().await.unwrap();

    let (kp, pk, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let addr = pk.into();
    let personal = b"Hello, World!".to_vec();

    let message = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: personal.clone(),
        },
    );

    let signature = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        Signature::new_secure(&message, kp),
    ));

    let result = cluster
        .verify_zklogin(
            personal,
            signature.as_ref().to_owned(),
            "TRANSACTION_DATA",
            addr,
        )
        .await
        .unwrap_err();

    assert_debug_snapshot!(result, @r###""[\"Failed to deserialize TransactionData from bytes\"]""###);
}

#[tokio::test]
async fn test_verify_invalid_transaction() {
    let cluster = FullCluster::new().await.unwrap();

    let (kp, pk, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let addr = pk.into();
    // Create bytes that are not valid TransactionData
    let invalid_tx_bytes = b"not a valid transaction".to_vec();

    // Create a signature for a real transaction (but we'll send invalid bytes)
    let rgp = cluster.onchain.get_reference_gas_price().await;
    let gas = cluster
        .onchain
        .fund_address_and_return_gas(rgp, Some(1000000), addr)
        .await;

    let tx = TestTransactionBuilder::new(addr, gas, rgp)
        .transfer_haneul(None, HaneulAddress::ZERO)
        .build();

    let message = IntentMessage::new(Intent::haneul_transaction(), tx);
    let signature = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        Signature::new_secure(&message, kp),
    ));

    let result = cluster
        .verify_zklogin(
            invalid_tx_bytes,
            signature.as_ref().to_owned(),
            "TRANSACTION_DATA",
            addr,
        )
        .await
        .unwrap_err();

    assert_debug_snapshot!(result, @r###""[\"Failed to deserialize TransactionData from bytes\"]""###);
}

#[tokio::test]
async fn test_verify_wrong_address() {
    let cluster = FullCluster::new().await.unwrap();

    let (kp, pk, inputs) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let _addr: HaneulAddress = pk.into();
    let personal = b"Hello, World!".to_vec();

    let message = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: personal.clone(),
        },
    );

    let signature = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        inputs.clone(),
        2,
        Signature::new_secure(&message, kp),
    ));

    let result = cluster
        .verify_zklogin(
            personal,
            signature.as_ref().to_owned(),
            "PERSONAL_MESSAGE",
            HaneulAddress::ZERO, // Wrong address
        )
        .await
        .unwrap_err();

    assert_debug_snapshot!(result, @r###""[\"Verification failed: Invalid address\"]""###);
}

#[tokio::test]
async fn test_verify_invalid_signature() {
    let cluster = FullCluster::new().await.unwrap();

    let (_, pk, _) =
        &load_test_vectors("../haneul-types/src/unit_tests/zklogin_test_vectors.json")[1];

    let addr: HaneulAddress = pk.into();
    let personal = b"Hello, World!".to_vec();

    // Create invalid signature bytes
    let invalid_signature = vec![0xFF; 100];

    let result = cluster
        .verify_zklogin(personal, invalid_signature, "PERSONAL_MESSAGE", addr)
        .await
        .unwrap_err();

    assert_debug_snapshot!(result, @r###""[\"Cannot parse signature\"]""###);
}

#[tokio::test]
async fn test_verify_not_zklogin_signature() {
    let cluster = FullCluster::new().await.unwrap();

    // Create a regular Ed25519 keypair
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    use haneul_types::crypto::HaneulKeyPair;
    use rand::SeedableRng;

    let keypair = HaneulKeyPair::Ed25519(Ed25519KeyPair::generate(
        &mut rand::rngs::StdRng::from_seed([1; 32]),
    ));
    let addr = HaneulAddress::from(&keypair.public());

    let personal = b"Hello, World!".to_vec();

    let message = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: personal.clone(),
        },
    );

    // Sign with regular Ed25519 signature
    let signature = GenericSignature::Signature(Signature::new_secure(&message, &keypair));

    let result = cluster
        .verify_zklogin(
            personal,
            signature.as_ref().to_owned(),
            "PERSONAL_MESSAGE",
            addr,
        )
        .await
        .unwrap_err();

    assert_debug_snapshot!(result, @r###""[\"Not a zkLogin signature\"]""###);
}
