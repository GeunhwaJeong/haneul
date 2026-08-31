// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use fastcrypto::encoding::{Encoding, Hex};
use fastcrypto::traits::EncodeDecodeBase64;
use haneul_config::{HANEUL_KEYSTORE_FILENAME, haneul_config_dir};
use haneul_rosetta::types::{CurveType, HaneulEnv, PrefundedAccount};
use haneul_rosetta::{HANEUL, RosettaOfflineServer, RosettaOnlineServer};
use haneul_rpc::client::Client as GrpcClient;
use haneul_rpc::client::HeadersInterceptor;
use haneul_rpc::proto::haneul::rpc::v2::GetServiceInfoRequest;
use haneul_types::base_types::HaneulAddress;
use haneul_types::crypto::{HaneulKeyPair, KeypairTraits, ToFromBytes};
use haneul_types::digests::{ChainIdentifier, CheckpointDigest};
use serde_json::{Value, json};
use tonic::metadata::Ascii;
use tonic::metadata::MetadataKey;
use tonic::metadata::MetadataValue;
use tracing::info;

#[derive(Parser)]
#[clap(name = "haneul-rosetta", rename_all = "kebab-case", author, version)]
pub enum RosettaServerCommand {
    GenerateRosettaCLIConfig {
        #[clap(long)]
        keystore_path: Option<PathBuf>,
        #[clap(long, default_value = "localnet")]
        env: HaneulEnv,
        #[clap(long, default_value = "http://rosetta-online:9002")]
        online_url: String,
        #[clap(long, default_value = "http://rosetta-offline:9003")]
        offline_url: String,
    },
    StartOnlineRemoteServer {
        #[clap(long, default_value = "localnet")]
        env: HaneulEnv,
        #[clap(long, default_value = "0.0.0.0:9002")]
        addr: SocketAddr,
        #[clap(long)]
        full_node_url: String,
        #[clap(long, default_value = "/data")]
        data_path: PathBuf,
        /// Additional gRPC header to send on every request to the full node as
        /// `<name>:<value>`. May be provided multiple times.
        #[clap(long = "haneul-grpc-header", value_parser = parse_grpc_header)]
        grpc_headers: Vec<(MetadataKey<Ascii>, MetadataValue<Ascii>)>,
    },
    StartOfflineServer {
        #[clap(long, default_value = "localnet")]
        env: HaneulEnv,
        #[clap(long, default_value = "0.0.0.0:9003")]
        addr: SocketAddr,
    },
}

impl RosettaServerCommand {
    async fn execute(self) -> Result<(), anyhow::Error> {
        match self {
            RosettaServerCommand::GenerateRosettaCLIConfig {
                keystore_path,
                env,
                online_url,
                offline_url,
            } => {
                let path = keystore_path
                    .unwrap_or_else(|| haneul_config_dir().unwrap().join(HANEUL_KEYSTORE_FILENAME));

                let prefunded_accounts = read_prefunded_account(&path)?;

                info!(
                    "Retrieved {} Haneul address from keystore file {:?}",
                    prefunded_accounts.len(),
                    &path
                );

                let mut config: Value =
                    serde_json::from_str(include_str!("../resources/rosetta_cli.json"))?;

                config
                    .as_object_mut()
                    .unwrap()
                    .insert("online_url".into(), json!(online_url));

                // Set network.
                let network = config.pointer_mut("/network").ok_or_else(|| {
                    anyhow!("Cannot find construction config in default config file.")
                })?;
                network
                    .as_object_mut()
                    .unwrap()
                    .insert("network".into(), json!(env));

                // Add prefunded accounts.
                let construction = config.pointer_mut("/construction").ok_or_else(|| {
                    anyhow!("Cannot find construction config in default config file.")
                })?;

                let construction = construction.as_object_mut().unwrap();
                construction.insert("prefunded_accounts".into(), json!(prefunded_accounts));
                construction.insert("offline_url".into(), json!(offline_url));

                let config_path = PathBuf::from(".").join("rosetta_cli.json");
                fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
                info!(
                    "Rosetta CLI configuration file is stored in {:?}",
                    config_path
                );

                let dsl_path = PathBuf::from(".").join("haneul.ros");
                let dsl = include_str!("../resources/haneul.ros");
                fs::write(
                    &dsl_path,
                    dsl.replace("{{haneul.env}}", json!(env).as_str().unwrap()),
                )?;
                info!("Rosetta DSL file is stored in {:?}", dsl_path);
            }
            RosettaServerCommand::StartOfflineServer { env, addr } => {
                info!("Starting Rosetta Offline Server.");
                let server = RosettaOfflineServer::new(env);
                server.serve(addr).await;
            }
            RosettaServerCommand::StartOnlineRemoteServer {
                env,
                addr,
                full_node_url,
                data_path,
                grpc_headers,
            } => {
                info!(
                    "Starting Rosetta Online Server with remote Haneul full node [{full_node_url}]."
                );
                let rosetta_path = data_path.join("rosetta_db");
                info!("Rosetta db path : {rosetta_path:?}");
                let mut client = GrpcClient::new(&full_node_url)
                    .map_err(|e| anyhow::anyhow!("Failed to create gRPC client: {}", e))?
                    .with_max_decoding_message_size(128 * 1024 * 1024);
                if !grpc_headers.is_empty() {
                    let mut headers = HeadersInterceptor::new();
                    for (name, value) in grpc_headers {
                        headers.headers_mut().insert(name, value);
                    }
                    client = client.with_headers(headers);
                }
                let chain_id = fetch_chain_id(&mut client).await?;
                let rosetta = RosettaOnlineServer::new(env, client, chain_id);
                rosetta.serve(addr).await;
            }
        };
        Ok(())
    }
}

async fn fetch_chain_id(client: &mut GrpcClient) -> Result<ChainIdentifier, anyhow::Error> {
    let response = client
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await?
        .into_inner();
    let digest = CheckpointDigest::from_str(response.chain_id())?;
    Ok(ChainIdentifier::from(digest))
}

fn parse_grpc_header(header: &str) -> Result<(MetadataKey<Ascii>, MetadataValue<Ascii>), String> {
    let (name, value) = header
        .split_once(':')
        .ok_or_else(|| "gRPC header must be in `<name>:<value>` format".to_string())?;

    let name = MetadataKey::from_bytes(name.as_bytes())
        .map_err(|err| format!("invalid gRPC header name `{name}`: {err}"))?;
    let mut value = MetadataValue::try_from(value)
        .map_err(|err| format!("invalid gRPC header value for `{name}`: {err}"))?;
    // Header values are likely auth tokens; keep them out of logs/debug output.
    value.set_sensitive(true);
    Ok((name, value))
}

/// This method reads the keypairs from the Haneul keystore to create the PrefundedAccount objects,
/// PrefundedAccount will be written to the rosetta-cli config file for testing.
///
fn read_prefunded_account(path: &Path) -> Result<Vec<PrefundedAccount>, anyhow::Error> {
    let reader = BufReader::new(File::open(path).unwrap());
    let kp_strings: Vec<String> = serde_json::from_reader(reader).unwrap();
    let keys = kp_strings
        .iter()
        .map(|kpstr| {
            let key = HaneulKeyPair::decode_base64(kpstr);
            key.map(|k| (HaneulAddress::from(&k.public()), k))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()
        .unwrap();

    Ok(keys
        .into_iter()
        .map(|(address, key)| {
            let (privkey, curve_type) = match key {
                HaneulKeyPair::Ed25519(k) => {
                    (Hex::encode(k.private().as_bytes()), CurveType::Edwards25519)
                }
                HaneulKeyPair::Secp256k1(k) => {
                    (Hex::encode(k.private().as_bytes()), CurveType::Secp256k1)
                }
                HaneulKeyPair::Secp256r1(k) => {
                    (Hex::encode(k.private().as_bytes()), CurveType::Secp256r1)
                }
            };
            PrefundedAccount {
                privkey,
                account_identifier: address.into(),
                curve_type,
                currency: HANEUL.clone(),
            }
        })
        .collect())
}

#[tokio::test]
async fn test_read_keystore() {
    use haneul_keys::keystore::{
        AccountKeystore, FileBasedKeystore, GenerateOptions, Keystore, LocalGenerate,
    };
    use haneul_types::crypto::SignatureScheme;

    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().join("haneul.keystore");
    let mut ks = Keystore::from(FileBasedKeystore::load_or_create(&path).unwrap());
    let key1 = ks
        .generate(
            None,
            GenerateOptions::Local(LocalGenerate {
                key_scheme: SignatureScheme::ED25519,
                derivation_path: None,
                word_length: None,
            }),
        )
        .await
        .unwrap();
    let key2 = ks
        .generate(
            None,
            GenerateOptions::Local(LocalGenerate {
                key_scheme: SignatureScheme::Secp256k1,
                derivation_path: None,
                word_length: None,
            }),
        )
        .await
        .unwrap();

    let accounts = read_prefunded_account(&path).unwrap();
    let acc_map = accounts
        .into_iter()
        .map(|acc| (acc.account_identifier.address, acc))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(2, acc_map.len());
    assert!(acc_map.contains_key(&key1.address));
    assert!(acc_map.contains_key(&key2.address));

    let acc1 = acc_map[&key1.address].clone();
    let acc2 = acc_map[&key2.address].clone();

    let schema1: SignatureScheme = acc1.curve_type.into();
    let schema2: SignatureScheme = acc2.curve_type.into();
    assert!(matches!(schema1, SignatureScheme::ED25519));
    assert!(matches!(schema2, SignatureScheme::Secp256k1));
}

#[test]
fn test_parse_grpc_header() {
    let (name, value) = parse_grpc_header("x-token:secret").unwrap();
    assert_eq!(name.as_str(), "x-token");
    assert_eq!(value.to_str().unwrap(), "secret");
    assert!(value.is_sensitive());

    // Values may legitimately contain `:` (only the first delimiter splits).
    let (name, value) = parse_grpc_header("authorization:Bearer a:b:c").unwrap();
    assert_eq!(name.as_str(), "authorization");
    assert_eq!(value.to_str().unwrap(), "Bearer a:b:c");

    assert!(parse_grpc_header("no-delimiter").is_err());
    assert!(parse_grpc_header("bad name:value").is_err());
    assert!(parse_grpc_header("name:invalid\nvalue").is_err());
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cmd: RosettaServerCommand = RosettaServerCommand::parse();

    let (_guard, _) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    cmd.execute().await
}
