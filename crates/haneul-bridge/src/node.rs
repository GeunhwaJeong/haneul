// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::action_executor::BridgeActionExecutor;
use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::config::{BridgeClientConfig, BridgeNodeConfig, WatchdogConfig};
use crate::crypto::BridgeAuthorityPublicKeyBytes;
use crate::eth_syncer::EthSyncer;
use crate::events::init_all_struct_tags;
use crate::haneul_bridge_watchdog::eth_bridge_status::EthBridgeStatus;
use crate::haneul_bridge_watchdog::eth_vault_balance::{EthereumVaultBalance, VaultAsset};
use crate::haneul_bridge_watchdog::haneul_bridge_status::HaneulBridgeStatus;
use crate::haneul_bridge_watchdog::metrics::WatchdogMetrics;
use crate::haneul_bridge_watchdog::total_supplies::TotalSupplies;
use crate::haneul_bridge_watchdog::{BridgeWatchDog, Observable};
use crate::haneul_client::HaneulBridgeClient;
use crate::haneul_syncer::HaneulSyncer;
use crate::metrics::BridgeMetrics;
use crate::monitor::{self, BridgeMonitor};
use crate::orchestrator::BridgeOrchestrator;
use crate::server::handler::BridgeRequestHandler;
use crate::server::{BridgeNodePublicMetadata, run_server};
use crate::storage::BridgeOrchestratorTables;
use crate::types::BridgeCommittee;
use crate::utils::{
    EthProvider, get_committee_voting_power_by_name, get_eth_contract_addresses,
    get_validator_names_by_pub_keys,
};
use alloy::primitives::Address as EthAddress;
use arc_swap::ArcSwap;
use haneul_types::Identifier;
use haneul_types::bridge::{
    BRIDGE_COMMITTEE_MODULE_NAME, BRIDGE_LIMITER_MODULE_NAME, BRIDGE_MODULE_NAME,
    BRIDGE_TREASURY_MODULE_NAME,
};
use haneul_types::event::EventID;
use haneullabs_common::ZipDebugEqIteratorExt;
use haneullabs_metrics::spawn_logged_monitored_task;
use std::collections::{BTreeMap, HashMap};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::info;

pub async fn run_bridge_node(
    config: BridgeNodeConfig,
    metadata: BridgeNodePublicMetadata,
    prometheus_registry: prometheus::Registry,
) -> anyhow::Result<JoinHandle<()>> {
    init_all_struct_tags();
    let metrics = Arc::new(BridgeMetrics::new(&prometheus_registry));
    let watchdog_config = config.watchdog_config.clone();
    let (server_config, client_config) = config.validate(metrics.clone()).await?;
    let haneul_chain_identifier = server_config
        .haneul_client
        .get_chain_identifier()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get haneul chain identifier: {:?}", e))?;
    let eth_chain_identifier = server_config
        .eth_client
        .get_chain_id()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get eth chain identifier: {:?}", e))?;
    prometheus_registry
        .register(haneullabs_metrics::bridge_uptime_metric(
            "bridge",
            metadata.version,
            &haneul_chain_identifier,
            &eth_chain_identifier.to_string(),
            client_config.is_some(),
        ))
        .unwrap();

    let committee = Arc::new(
        server_config
            .haneul_client
            .get_bridge_committee()
            .await
            .expect("Failed to get committee"),
    );
    let mut handles = vec![];

    // Start watchdog
    let eth_provider = server_config.eth_client.provider();
    let eth_bridge_proxy_address = server_config.eth_bridge_proxy_address;
    let haneul_client = server_config.haneul_client.clone();
    handles.push(spawn_logged_monitored_task!(start_watchdog(
        watchdog_config,
        &prometheus_registry,
        eth_provider,
        eth_bridge_proxy_address,
        haneul_client
    )));

    // Update voting right metrics
    // Before reconfiguration happens we only set it once when the node starts
    let haneul_system = server_config
        .haneul_client
        .grpc_client()
        .get_system_state_summary(None)
        .await?;

    // Start Client
    if let Some(client_config) = client_config {
        let committee_keys_to_names =
            Arc::new(get_validator_names_by_pub_keys(&committee, &haneul_system).await);
        let client_components = start_client_components(
            client_config,
            committee.clone(),
            committee_keys_to_names,
            metrics.clone(),
        )
        .await?;
        handles.extend(client_components);
    }

    let committee_name_mapping =
        get_committee_voting_power_by_name(&committee, &haneul_system).await;
    for (name, voting_power) in committee_name_mapping.into_iter() {
        metrics
            .current_bridge_voting_rights
            .with_label_values(&[name.as_str()])
            .set(voting_power as i64);
    }

    // Start Server
    let socket_address = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        server_config.server_listen_port,
    );
    Ok(run_server(
        &socket_address,
        BridgeRequestHandler::new(
            server_config.key,
            server_config.haneul_client,
            server_config.eth_client,
            server_config.approved_governance_actions,
        ),
        metrics,
        Arc::new(metadata),
    ))
}

async fn start_watchdog(
    watchdog_config: Option<WatchdogConfig>,
    registry: &prometheus::Registry,
    eth_provider: EthProvider,
    eth_bridge_proxy_address: EthAddress,
    haneul_client: Arc<HaneulBridgeClient>,
) {
    let watchdog_metrics = WatchdogMetrics::new(registry);
    let (
        _committee_address,
        _limiter_address,
        vault_address,
        _config_address,
        weth_address,
        usdt_address,
        wbtc_address,
        lbtc_address,
    ) = get_eth_contract_addresses(eth_bridge_proxy_address, eth_provider.clone())
        .await
        .unwrap_or_else(|e| panic!("get_eth_contract_addresses should not fail: {}", e));

    // If vault_address is zero (can happen due to storage layout mismatch during upgrades),
    // skip vault balance monitoring but allow node to start for signing server functionality.
    let vault_monitoring_enabled = !vault_address.is_zero() && !weth_address.is_zero();
    if !vault_monitoring_enabled {
        tracing::warn!(
            "Vault address or token addresses are zero - skipping vault balance monitoring. \
            This is expected during storage layout mismatch recovery."
        );
    }

    let eth_bridge_status = EthBridgeStatus::new(
        eth_provider.clone(),
        eth_bridge_proxy_address,
        watchdog_metrics.eth_bridge_paused.clone(),
    );

    let haneul_bridge_status = HaneulBridgeStatus::new(
        haneul_client.clone(),
        watchdog_metrics.haneul_bridge_paused.clone(),
    );

    let mut observables: Vec<Box<dyn Observable + Send + Sync>> =
        vec![Box::new(eth_bridge_status), Box::new(haneul_bridge_status)];

    // Add vault balance monitors only when addresses are valid
    if vault_monitoring_enabled {
        let eth_vault_balance = EthereumVaultBalance::new(
            eth_provider.clone(),
            vault_address,
            weth_address,
            VaultAsset::WETH,
            watchdog_metrics.eth_vault_balance.clone(),
        )
        .await
        .unwrap_or_else(|e| panic!("Failed to create eth vault balance: {}", e));

        observables.push(Box::new(eth_vault_balance));

        if !usdt_address.is_zero() {
            let usdt_vault_balance = EthereumVaultBalance::new(
                eth_provider.clone(),
                vault_address,
                usdt_address,
                VaultAsset::USDT,
                watchdog_metrics.usdt_vault_balance.clone(),
            )
            .await
            .unwrap_or_else(|e| panic!("Failed to create usdt vault balance: {}", e));
            observables.push(Box::new(usdt_vault_balance));
        }

        if !wbtc_address.is_zero() {
            let wbtc_vault_balance = EthereumVaultBalance::new(
                eth_provider.clone(),
                vault_address,
                wbtc_address,
                VaultAsset::WBTC,
                watchdog_metrics.wbtc_vault_balance.clone(),
            )
            .await
            .unwrap_or_else(|e| panic!("Failed to create wbtc vault balance: {}", e));
            observables.push(Box::new(wbtc_vault_balance));
        }

        if !lbtc_address.is_zero() {
            let lbtc_vault_balance = EthereumVaultBalance::new(
                eth_provider,
                vault_address,
                lbtc_address,
                VaultAsset::LBTC,
                watchdog_metrics.lbtc_vault_balance.clone(),
            )
            .await
            .unwrap_or_else(|e| panic!("Failed to create lbtc vault balance: {}", e));
            observables.push(Box::new(lbtc_vault_balance));
        }
    }

    if let Some(watchdog_config) = watchdog_config
        && !watchdog_config.total_supplies.is_empty()
    {
        let total_supplies = TotalSupplies::new(
            haneul_client.grpc_client().clone().into_inner(),
            watchdog_config.total_supplies,
            watchdog_metrics.total_supplies.clone(),
        );
        observables.push(Box::new(total_supplies));
    }

    BridgeWatchDog::new(observables).run().await
}

// TODO: is there a way to clean up the overrides after it's stored in DB?
async fn start_client_components(
    client_config: BridgeClientConfig,
    committee: Arc<BridgeCommittee>,
    committee_keys_to_names: Arc<BTreeMap<BridgeAuthorityPublicKeyBytes, String>>,
    metrics: Arc<BridgeMetrics>,
) -> anyhow::Result<Vec<JoinHandle<()>>> {
    let store: std::sync::Arc<BridgeOrchestratorTables> =
        BridgeOrchestratorTables::new(&client_config.db_path.join("client"));
    let haneul_modules_to_watch = get_haneul_modules_to_watch(
        &store,
        client_config.haneul_bridge_module_last_processed_event_id_override,
    );

    let eth_contracts_to_watch = get_eth_contracts_to_watch(
        &store,
        &client_config.eth_contracts,
        client_config.eth_contracts_start_block_fallback,
        client_config.eth_contracts_start_block_override,
    );

    let haneul_client = client_config.haneul_client.clone();

    let last_processed_bridge_event_id = haneul_modules_to_watch
        .get(&BRIDGE_MODULE_NAME.to_owned())
        .and_then(|opt| *opt);

    let next_sequence_number = get_next_sequence_number(
        &store,
        &haneul_client,
        last_processed_bridge_event_id,
        client_config.haneul_bridge_next_sequence_number_override,
    )
    .await;

    let mut all_handles = vec![];
    let (task_handles, eth_events_rx, _) =
        EthSyncer::new(client_config.eth_client.clone(), eth_contracts_to_watch)
            .run(metrics.clone())
            .await
            .expect("Failed to start eth syncer");
    all_handles.extend(task_handles);

    let (task_handles, haneul_grpc_events_rx) = HaneulSyncer::new(
        client_config.haneul_client,
        haneul_modules_to_watch,
        metrics.clone(),
    )
    .run_grpc(
        client_config.haneul_bridge_chain_id,
        next_sequence_number,
        Duration::from_secs(2),
        10,
    )
    .await
    .expect("Failed to start haneul syncer");
    all_handles.extend(task_handles);

    let bridge_auth_agg = Arc::new(ArcSwap::from(Arc::new(BridgeAuthorityAggregator::new(
        committee,
        metrics.clone(),
        committee_keys_to_names,
    ))));
    // TODO: should we use one query instead of two?
    let haneul_token_type_tags = haneul_client.get_token_id_map().await.unwrap();
    let is_bridge_paused = haneul_client.is_bridge_paused().await.unwrap();

    let (bridge_pause_tx, bridge_pause_rx) = tokio::sync::watch::channel(is_bridge_paused);

    let (eth_monitor_tx, eth_monitor_rx) = haneullabs_metrics::metered_channel::channel(
        10000,
        &haneullabs_metrics::get_metrics()
            .unwrap()
            .channel_inflight
            .with_label_values(&["eth_monitor_queue"]),
    );

    let haneul_token_type_tags = Arc::new(ArcSwap::from(Arc::new(haneul_token_type_tags)));
    let bridge_action_executor = BridgeActionExecutor::new(
        haneul_client.clone(),
        bridge_auth_agg.clone(),
        store.clone(),
        client_config.key,
        client_config.haneul_address,
        client_config.gas_object_ref.0,
        haneul_token_type_tags.clone(),
        bridge_pause_rx,
        metrics.clone(),
    )
    .await;

    let (haneul_monitor_tx, haneul_monitor_rx) = haneullabs_metrics::metered_channel::channel(
        10000,
        &haneullabs_metrics::get_metrics()
            .unwrap()
            .channel_inflight
            .with_label_values(&["haneul_monitor_queue"]),
    );
    tokio::spawn(monitor::subscribe_bridge_events(
        haneul_client.grpc_client().clone().into_inner(),
        haneul_monitor_tx,
    ));
    let monitor = BridgeMonitor::new(
        haneul_client.clone(),
        haneul_monitor_rx,
        eth_monitor_rx,
        bridge_auth_agg.clone(),
        bridge_pause_tx,
        haneul_token_type_tags,
        metrics.clone(),
    );
    all_handles.push(spawn_logged_monitored_task!(monitor.run()));

    let orchestrator = BridgeOrchestrator::new(
        haneul_client,
        haneul_grpc_events_rx,
        eth_events_rx,
        store.clone(),
        eth_monitor_tx,
        metrics,
    );

    all_handles.extend(orchestrator.run_with_grpc(bridge_action_executor).await);
    Ok(all_handles)
}

async fn get_next_sequence_number<C: crate::haneul_client::HaneulClientInner>(
    store: &BridgeOrchestratorTables,
    haneul_client: &crate::haneul_client::HaneulClient<C>,
    last_processed_bridge_event_id: Option<EventID>,
    next_sequence_number_override: Option<u64>,
) -> u64 {
    if let Some(next_sequence_number_override) = next_sequence_number_override {
        info!("Overriding next sequence number to {next_sequence_number_override}",);
        return next_sequence_number_override;
    }

    if let Ok(Some(sequence_number)) = store.get_haneul_sequence_number_cursor() {
        info!("Using sequence number {sequence_number} from storage",);
        return sequence_number;
    }

    if let Some(event_id) = last_processed_bridge_event_id {
        match haneul_client
            .get_sequence_number_from_event_id(event_id)
            .await
        {
            Ok(Some(sequence_number)) => {
                let next = sequence_number + 1;
                info!(
                    ?event_id,
                    last_processed_seq = sequence_number,
                    next_seq_to_read = next,
                    "Migrated from legacy event cursor to sequence number cursor"
                );
                return next;
            }
            Ok(None) => {
                info!(
                    ?event_id,
                    "Could not extract sequence number from legacy event cursor, starting from 0"
                );
            }
            Err(e) => {
                info!(
                    ?event_id,
                    ?e,
                    "Failed to get sequence number from legacy event cursor, starting from 0"
                );
            }
        }
    }

    info!("No cursor found for gRPC syncer, starting from sequence number 0");
    0
}

fn get_haneul_modules_to_watch(
    store: &std::sync::Arc<BridgeOrchestratorTables>,
    haneul_bridge_module_last_processed_event_id_override: Option<EventID>,
) -> HashMap<Identifier, Option<EventID>> {
    let haneul_bridge_modules = vec![
        BRIDGE_MODULE_NAME.to_owned(),
        BRIDGE_COMMITTEE_MODULE_NAME.to_owned(),
        BRIDGE_TREASURY_MODULE_NAME.to_owned(),
        BRIDGE_LIMITER_MODULE_NAME.to_owned(),
    ];
    if let Some(cursor) = haneul_bridge_module_last_processed_event_id_override {
        info!(
            "Overriding cursor for haneul bridge modules to {:?}",
            cursor
        );
        return HashMap::from_iter(
            haneul_bridge_modules
                .iter()
                .map(|module| (module.clone(), Some(cursor))),
        );
    }

    let haneul_bridge_module_stored_cursor = store
        .get_haneul_event_cursors(&haneul_bridge_modules)
        .expect("Failed to get eth haneul event cursors from storage");
    let mut haneul_modules_to_watch = HashMap::new();
    for (module_identifier, cursor) in haneul_bridge_modules
        .iter()
        .zip_debug_eq(haneul_bridge_module_stored_cursor)
    {
        if cursor.is_none() {
            info!(
                "No cursor found for haneul bridge module {} in storage or config override, query start from the beginning.",
                module_identifier
            );
        }
        haneul_modules_to_watch.insert(module_identifier.clone(), cursor);
    }
    haneul_modules_to_watch
}

fn get_eth_contracts_to_watch(
    store: &std::sync::Arc<BridgeOrchestratorTables>,
    eth_contracts: &[EthAddress],
    eth_contracts_start_block_fallback: u64,
    eth_contracts_start_block_override: Option<u64>,
) -> HashMap<EthAddress, u64> {
    let stored_eth_cursors = store
        .get_eth_event_cursors(eth_contracts)
        .expect("Failed to get eth event cursors from storage");
    let mut eth_contracts_to_watch = HashMap::new();
    for (contract, stored_cursor) in eth_contracts.iter().zip_debug_eq(stored_eth_cursors) {
        // start block precedence:
        // eth_contracts_start_block_override > stored cursor > eth_contracts_start_block_fallback
        match (eth_contracts_start_block_override, stored_cursor) {
            (Some(override_), _) => {
                eth_contracts_to_watch.insert(*contract, override_);
                info!(
                    "Overriding cursor for eth bridge contract {} to {}. Stored cursor: {:?}",
                    contract, override_, stored_cursor
                );
            }
            (None, Some(stored_cursor)) => {
                // +1: The stored value is the last block that was processed, so we start from the next block.
                eth_contracts_to_watch.insert(*contract, stored_cursor + 1);
            }
            (None, None) => {
                // If no cursor is found, start from the fallback block.
                eth_contracts_to_watch.insert(*contract, eth_contracts_start_block_fallback);
            }
        }
    }
    eth_contracts_to_watch
}

#[cfg(test)]
mod tests {
    use alloy::primitives::Address as EthAddress;
    use alloy::primitives::U160;
    use prometheus::Registry;

    use super::*;
    use crate::config::BridgeNodeConfig;
    use crate::config::EthConfig;
    use crate::config::HaneulConfig;
    use crate::config::default_ed25519_key_pair;
    use crate::e2e_tests::test_utils::BridgeTestCluster;
    use crate::e2e_tests::test_utils::BridgeTestClusterBuilder;
    use crate::utils::wait_for_server_to_be_up;
    use fastcrypto::secp256k1::Secp256k1KeyPair;
    use haneul_config::local_ip_utils::get_available_port;
    use haneul_types::base_types::HaneulAddress;
    use haneul_types::bridge::BridgeChainId;
    use haneul_types::crypto::EncodeDecodeBase64;
    use haneul_types::crypto::HaneulKeyPair;
    use haneul_types::crypto::KeypairTraits;
    use haneul_types::crypto::get_key_pair;
    use haneul_types::digests::TransactionDigest;
    use haneul_types::event::EventID;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_get_eth_contracts_to_watch() {
        telemetry_subscribers::init_for_testing();
        let temp_dir = tempfile::tempdir().unwrap();
        let eth_contracts = vec![
            EthAddress::from(U160::from(1)),
            EthAddress::from(U160::from(2)),
        ];
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        // No override, no watermark found in DB, use fallback
        let contracts = get_eth_contracts_to_watch(&store, &eth_contracts, 10, None);
        assert_eq!(
            contracts,
            vec![(eth_contracts[0], 10), (eth_contracts[1], 10)]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );

        // no watermark found in DB, use override
        let contracts = get_eth_contracts_to_watch(&store, &eth_contracts, 10, Some(420));
        assert_eq!(
            contracts,
            vec![(eth_contracts[0], 420), (eth_contracts[1], 420)]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );

        store
            .update_eth_event_cursor(eth_contracts[0], 100)
            .unwrap();
        store
            .update_eth_event_cursor(eth_contracts[1], 102)
            .unwrap();

        // No override, found watermarks in DB, use +1
        let contracts = get_eth_contracts_to_watch(&store, &eth_contracts, 10, None);
        assert_eq!(
            contracts,
            vec![(eth_contracts[0], 101), (eth_contracts[1], 103)]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );

        // use override
        let contracts = get_eth_contracts_to_watch(&store, &eth_contracts, 10, Some(200));
        assert_eq!(
            contracts,
            vec![(eth_contracts[0], 200), (eth_contracts[1], 200)]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_starting_bridge_node() {
        telemetry_subscribers::init_for_testing();
        let bridge_test_cluster = setup().await;
        let kp = bridge_test_cluster.bridge_authority_key(0);

        // prepare node config (server only)
        let tmp_dir = tempdir().unwrap().keep();
        let authority_key_path = "test_starting_bridge_node_bridge_authority_key";
        let server_listen_port = get_available_port("127.0.0.1");
        let base64_encoded = kp.encode_base64();
        std::fs::write(tmp_dir.join(authority_key_path), base64_encoded).unwrap();

        let config = BridgeNodeConfig {
            server_listen_port,
            metrics_port: get_available_port("127.0.0.1"),
            bridge_authority_key_path: tmp_dir.join(authority_key_path),
            haneul: HaneulConfig {
                haneul_rpc_url: bridge_test_cluster.haneul_rpc_url(),
                haneul_bridge_chain_id: BridgeChainId::HaneulCustom as u8,
                bridge_client_key_path: None,
                bridge_client_gas_object: None,
                haneul_bridge_module_last_processed_event_id_override: None,
                haneul_bridge_next_sequence_number_override: None,
            },
            eth: EthConfig {
                eth_rpc_url: None,
                eth_rpc_urls: Some(vec![bridge_test_cluster.eth_rpc_url()]),
                eth_rpc_quorum: 1,
                eth_health_check_interval_secs: 300,
                eth_bridge_proxy_address: bridge_test_cluster.haneul_bridge_address(),
                eth_bridge_chain_id: BridgeChainId::EthCustom as u8,
                eth_contracts_start_block_fallback: None,
                eth_contracts_start_block_override: None,
            },
            approved_governance_actions: vec![],
            run_client: false,
            db_path: None,
            metrics_key_pair: default_ed25519_key_pair(),
            metrics: None,
            watchdog_config: None,
        };
        // Spawn bridge node in memory
        let _handle = run_bridge_node(
            config,
            BridgeNodePublicMetadata::empty_for_testing(),
            Registry::new(),
        )
        .await
        .unwrap();

        let server_url = format!("http://127.0.0.1:{}", server_listen_port);
        // Now we expect to see the server to be up and running.
        let res = wait_for_server_to_be_up(server_url, 5).await;
        res.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_starting_bridge_node_with_client() {
        telemetry_subscribers::init_for_testing();
        let bridge_test_cluster = setup().await;
        let kp = bridge_test_cluster.bridge_authority_key(0);

        // prepare node config (server + client)
        let tmp_dir = tempdir().unwrap().keep();
        let db_path = tmp_dir.join("test_starting_bridge_node_with_client_db");
        let authority_key_path = "test_starting_bridge_node_with_client_bridge_authority_key";
        let server_listen_port = get_available_port("127.0.0.1");

        let base64_encoded = kp.encode_base64();
        std::fs::write(tmp_dir.join(authority_key_path), base64_encoded).unwrap();

        let client_haneul_address = HaneulAddress::from(kp.public());
        let sender_address = bridge_test_cluster.haneul_user_address();
        // send some gas to this address
        bridge_test_cluster
            .test_cluster
            .inner
            .transfer_haneul_must_exceed(sender_address, client_haneul_address, 1000000000)
            .await;

        let config = BridgeNodeConfig {
            server_listen_port,
            metrics_port: get_available_port("127.0.0.1"),
            bridge_authority_key_path: tmp_dir.join(authority_key_path),
            haneul: HaneulConfig {
                haneul_rpc_url: bridge_test_cluster.haneul_rpc_url(),
                haneul_bridge_chain_id: BridgeChainId::HaneulCustom as u8,
                bridge_client_key_path: None,
                bridge_client_gas_object: None,
                haneul_bridge_module_last_processed_event_id_override: Some(EventID {
                    tx_digest: TransactionDigest::random(),
                    event_seq: 0,
                }),
                haneul_bridge_next_sequence_number_override: None,
            },
            eth: EthConfig {
                eth_rpc_url: None,
                eth_rpc_urls: Some(vec![bridge_test_cluster.eth_rpc_url()]),
                eth_rpc_quorum: 1,
                eth_health_check_interval_secs: 300,
                eth_bridge_proxy_address: bridge_test_cluster.haneul_bridge_address(),
                eth_bridge_chain_id: BridgeChainId::EthCustom as u8,
                eth_contracts_start_block_fallback: Some(0),
                eth_contracts_start_block_override: None,
            },
            approved_governance_actions: vec![],
            run_client: true,
            db_path: Some(db_path),
            metrics_key_pair: default_ed25519_key_pair(),
            metrics: None,
            watchdog_config: None,
        };
        // Spawn bridge node in memory
        let _handle = run_bridge_node(
            config,
            BridgeNodePublicMetadata::empty_for_testing(),
            Registry::new(),
        )
        .await
        .unwrap();

        let server_url = format!("http://127.0.0.1:{}", server_listen_port);
        // Now we expect to see the server to be up and running.
        // client components are spawned earlier than server, so as long as the server is up,
        // we know the client components are already running.
        let res = wait_for_server_to_be_up(server_url, 5).await;
        res.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_starting_bridge_node_with_client_and_separate_client_key() {
        telemetry_subscribers::init_for_testing();
        let bridge_test_cluster = setup().await;
        let kp = bridge_test_cluster.bridge_authority_key(0);

        // prepare node config (server + client)
        let tmp_dir = tempdir().unwrap().keep();
        let db_path =
            tmp_dir.join("test_starting_bridge_node_with_client_and_separate_client_key_db");
        let authority_key_path =
            "test_starting_bridge_node_with_client_and_separate_client_key_bridge_authority_key";
        let server_listen_port = get_available_port("127.0.0.1");

        // prepare bridge authority key
        let base64_encoded = kp.encode_base64();
        std::fs::write(tmp_dir.join(authority_key_path), base64_encoded).unwrap();

        // prepare bridge client key
        let (_, kp): (_, Secp256k1KeyPair) = get_key_pair();
        let kp = HaneulKeyPair::from(kp);
        let client_key_path =
            "test_starting_bridge_node_with_client_and_separate_client_key_bridge_client_key";
        std::fs::write(tmp_dir.join(client_key_path), kp.encode_base64()).unwrap();
        let client_haneul_address = HaneulAddress::from(&kp.public());
        let sender_address = bridge_test_cluster.haneul_user_address();
        // send some gas to this address
        let gas_obj = bridge_test_cluster
            .test_cluster
            .inner
            .transfer_haneul_must_exceed(sender_address, client_haneul_address, 1000000000)
            .await;

        let config = BridgeNodeConfig {
            server_listen_port,
            metrics_port: get_available_port("127.0.0.1"),
            bridge_authority_key_path: tmp_dir.join(authority_key_path),
            haneul: HaneulConfig {
                haneul_rpc_url: bridge_test_cluster.haneul_rpc_url(),
                haneul_bridge_chain_id: BridgeChainId::HaneulCustom as u8,
                bridge_client_key_path: Some(tmp_dir.join(client_key_path)),
                bridge_client_gas_object: Some(gas_obj),
                haneul_bridge_module_last_processed_event_id_override: Some(EventID {
                    tx_digest: TransactionDigest::random(),
                    event_seq: 0,
                }),
                haneul_bridge_next_sequence_number_override: None,
            },
            eth: EthConfig {
                eth_rpc_url: None,
                eth_rpc_urls: Some(vec![bridge_test_cluster.eth_rpc_url()]),
                eth_rpc_quorum: 1,
                eth_health_check_interval_secs: 300,
                eth_bridge_proxy_address: bridge_test_cluster.haneul_bridge_address(),
                eth_bridge_chain_id: BridgeChainId::EthCustom as u8,
                eth_contracts_start_block_fallback: Some(0),
                eth_contracts_start_block_override: Some(0),
            },
            approved_governance_actions: vec![],
            run_client: true,
            db_path: Some(db_path),
            metrics_key_pair: default_ed25519_key_pair(),
            metrics: None,
            watchdog_config: None,
        };
        // Spawn bridge node in memory
        let _handle = run_bridge_node(
            config,
            BridgeNodePublicMetadata::empty_for_testing(),
            Registry::new(),
        )
        .await
        .unwrap();

        let server_url = format!("http://127.0.0.1:{}", server_listen_port);
        // Now we expect to see the server to be up and running.
        // client components are spawned earlier than server, so as long as the server is up,
        // we know the client components are already running.
        let res = wait_for_server_to_be_up(server_url, 5).await;
        res.unwrap();
    }

    async fn setup() -> BridgeTestCluster {
        BridgeTestClusterBuilder::new()
            .with_eth_env(true)
            .with_bridge_cluster(false)
            .with_num_validators(2)
            .build()
            .await
    }
}
