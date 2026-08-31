// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! `BridgeOrchestrator` is the component that:
//! 1. monitors Haneul and Ethereum events with the help of `HaneulSyncer` and `EthSyncer`
//! 2. updates WAL table and cursor tables
//! 2. hands actions to `BridgeExecutor` for execution

use crate::abi::EthBridgeEvent;
use crate::action_executor::{
    BridgeActionExecutionWrapper, BridgeActionExecutorTrait, submit_to_executor,
};
use crate::events::HaneulBridgeEvent;
use crate::haneul_client::{HaneulClient, HaneulClientInner};
use crate::haneul_syncer::GrpcSyncedEvents;
use crate::metrics::BridgeMetrics;
use crate::storage::BridgeOrchestratorTables;
use crate::types::EthLog;
use alloy::primitives::Address as EthAddress;
use haneullabs_common::ZipDebugEqIteratorExt;
use haneullabs_metrics::spawn_logged_monitored_task;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info};

pub struct BridgeOrchestrator<C> {
    _haneul_client: Arc<HaneulClient<C>>,
    haneul_grpc_events_rx:
        haneullabs_metrics::metered_channel::Receiver<(u64, Vec<HaneulBridgeEvent>)>,
    eth_events_rx: haneullabs_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
    store: Arc<BridgeOrchestratorTables>,
    eth_monitor_tx: haneullabs_metrics::metered_channel::Sender<EthBridgeEvent>,
    metrics: Arc<BridgeMetrics>,
}

impl<C> BridgeOrchestrator<C>
where
    C: HaneulClientInner + 'static,
{
    pub fn new(
        haneul_client: Arc<HaneulClient<C>>,
        haneul_grpc_events_rx: haneullabs_metrics::metered_channel::Receiver<(
            u64,
            Vec<HaneulBridgeEvent>,
        )>,
        eth_events_rx: haneullabs_metrics::metered_channel::Receiver<(
            EthAddress,
            u64,
            Vec<EthLog>,
        )>,
        store: Arc<BridgeOrchestratorTables>,
        eth_monitor_tx: haneullabs_metrics::metered_channel::Sender<EthBridgeEvent>,
        metrics: Arc<BridgeMetrics>,
    ) -> Self {
        Self {
            _haneul_client: haneul_client,
            haneul_grpc_events_rx,
            eth_events_rx,
            store,
            eth_monitor_tx,
            metrics,
        }
    }

    pub async fn run_with_grpc(
        self,
        bridge_action_executor: impl BridgeActionExecutorTrait,
    ) -> Vec<JoinHandle<()>> {
        tracing::info!("Starting BridgeOrchestrator with gRPC syncer");
        let mut task_handles = vec![];
        let store_clone = self.store.clone();

        // Spawn BridgeActionExecutor
        let (handles, executor_sender) = bridge_action_executor.run();
        task_handles.extend(handles);
        let executor_sender_clone = executor_sender.clone();
        let metrics_clone = self.metrics.clone();

        task_handles.push(spawn_logged_monitored_task!(Self::run_haneul_grpc_watcher(
            store_clone,
            executor_sender_clone,
            self.haneul_grpc_events_rx,
            metrics_clone,
        )));
        let store_clone = self.store.clone();

        // Re-submit pending actions to executor
        let actions = store_clone
            .get_all_pending_actions()
            .into_values()
            .collect::<Vec<_>>();
        for action in actions {
            submit_to_executor(&executor_sender, action)
                .await
                .expect("Submit to executor should not fail");
        }

        let metrics_clone = self.metrics.clone();
        task_handles.push(spawn_logged_monitored_task!(Self::run_eth_watcher(
            store_clone,
            executor_sender,
            self.eth_events_rx,
            self.eth_monitor_tx,
            metrics_clone,
        )));

        task_handles
    }

    pub async fn run_haneul_grpc_watcher(
        store: Arc<BridgeOrchestratorTables>,
        executor_tx: haneullabs_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut haneul_grpc_events_rx: haneullabs_metrics::metered_channel::Receiver<GrpcSyncedEvents>,
        metrics: Arc<BridgeMetrics>,
    ) {
        info!("Starting haneul gRPC watcher task");
        while let Some((last_seq_num, events)) = haneul_grpc_events_rx.recv().await {
            if events.is_empty() {
                continue;
            }
            info!(
                "Received {} Haneul events: last_seq_num={}",
                events.len(),
                last_seq_num
            );
            metrics
                .haneul_watcher_received_events
                .inc_by(events.len() as u64);

            let mut actions = vec![];
            for bridge_event in events {
                info!("Observed Haneul bridge event (gRPC): {:?}", bridge_event);

                // Convert to action using the same flow as JSON-RPC watcher
                if let Some(mut action) = bridge_event.try_into_bridge_action() {
                    metrics.last_observed_actions_seq_num.with_label_values(&[
                        action.chain_id().to_string().as_str(),
                        action.action_type().to_string().as_str(),
                    ]);

                    action = action.update_to_token_transfer();
                    actions.push(action);
                }
            }

            if !actions.is_empty() {
                info!(
                    "Received {} actions from Haneul gRPC: {:?}",
                    actions.len(),
                    actions
                );
                metrics
                    .haneul_watcher_received_actions
                    .inc_by(actions.len() as u64);
                // Write action to pending WAL
                store
                    .insert_pending_actions(&actions)
                    .expect("Store operation should not fail");
                for action in actions {
                    submit_to_executor(&executor_tx, action)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }

            // Store the sequence number cursor
            store
                .update_haneul_sequence_number_cursor(last_seq_num)
                .expect("Store operation should not fail");
        }
        panic!("Haneul gRPC event channel was closed unexpectedly");
    }

    async fn run_eth_watcher(
        store: Arc<BridgeOrchestratorTables>,
        executor_tx: haneullabs_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut eth_events_rx: haneullabs_metrics::metered_channel::Receiver<(
            alloy::primitives::Address,
            u64,
            Vec<EthLog>,
        )>,
        eth_monitor_tx: haneullabs_metrics::metered_channel::Sender<EthBridgeEvent>,
        metrics: Arc<BridgeMetrics>,
    ) {
        info!("Starting eth watcher task");
        while let Some((contract, end_block, logs)) = eth_events_rx.recv().await {
            if logs.is_empty() {
                store
                    .update_eth_event_cursor(contract, end_block)
                    .expect("Store operation should not fail");
                continue;
            }

            info!("Received {} Eth events", logs.len());
            metrics
                .eth_watcher_received_events
                .inc_by(logs.len() as u64);

            let bridge_events = logs
                .iter()
                .map(EthBridgeEvent::try_from_eth_log)
                .collect::<Vec<_>>();

            let mut actions = vec![];
            for (log, opt_bridge_event) in logs.iter().zip_debug_eq(bridge_events) {
                if opt_bridge_event.is_none() {
                    // TODO: we probably should not miss any events, log for now.
                    metrics.eth_watcher_unrecognized_events.inc();
                    error!("Eth event not recognized: {:?}", log);
                    continue;
                }
                // Unwrap safe: checked above
                let bridge_event = opt_bridge_event.unwrap();
                info!("Observed Eth bridge event: {:?}", bridge_event);

                // Send event to monitor
                eth_monitor_tx
                    .send(bridge_event.clone())
                    .await
                    .expect("Sending event to monitor channel should not fail");

                match bridge_event.try_into_bridge_action(log.tx_hash, log.log_index_in_tx) {
                    Ok(Some(action)) => {
                        metrics.last_observed_actions_seq_num.with_label_values(&[
                            action.chain_id().to_string().as_str(),
                            action.action_type().to_string().as_str(),
                        ]);
                        actions.push(action)
                    }
                    Ok(None) => {}
                    Err(e) => {
                        error!(eth_tx_hash=?log.tx_hash, eth_event_index=?log.log_index_in_tx, "Error converting EthBridgeEvent to BridgeAction: {:?}", e);
                    }
                }
            }
            if !actions.is_empty() {
                info!("Received {} actions from Eth: {:?}", actions.len(), actions);
                metrics
                    .eth_watcher_received_actions
                    .inc_by(actions.len() as u64);
                // Write action to pending WAL
                store
                    .insert_pending_actions(&actions)
                    .expect("Store operation should not fail");
                // Execution will remove the pending actions from DB when the action is completed.
                for action in actions {
                    submit_to_executor(&executor_tx, action)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }

            store
                .update_eth_event_cursor(contract, end_block)
                .expect("Store operation should not fail");
        }
        panic!("Eth event channel was closed");
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        test_utils::{get_test_eth_to_haneul_bridge_action, get_test_log_and_action},
        types::BridgeActionDigest,
    };
    use alloy::primitives::TxHash;
    use haneul_types::Identifier;
    use prometheus::Registry;
    use std::str::FromStr;

    use super::*;
    use crate::events::HaneulBridgeEvent;
    use crate::events::init_all_struct_tags;
    use crate::test_utils::get_test_haneul_to_eth_bridge_action;
    use crate::{
        events::tests::get_test_haneul_event_and_action, haneul_mock_client::HaneulMockClient,
    };

    #[tokio::test]
    async fn test_eth_watcher_task() {
        // Note: this test may fail because of the following reasons:
        // 1. Log and BridgeAction returned from `get_test_log_and_action` are not in sync
        // 2. Log returned from `get_test_log_and_action` is not parseable log (not abigen!, check abi.rs)

        let (
            _haneul_grpc_events_tx,
            haneul_grpc_events_rx,
            eth_events_tx,
            eth_events_rx,
            eth_monitor_tx,
            _eth_monitor_rx,
            haneul_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();
        // start orchestrator
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(haneul_client),
            haneul_grpc_events_rx,
            eth_events_rx,
            store.clone(),
            eth_monitor_tx,
            metrics,
        )
        .run_with_grpc(executor)
        .await;
        let address = EthAddress::random();
        let (log, bridge_action) = get_test_log_and_action(address, TxHash::random(), 10);
        let log_index_in_tx = 10;
        let log_block_num = log.block_number.unwrap();
        let eth_log = EthLog {
            log: log.clone(),
            tx_hash: log.transaction_hash.unwrap(),
            block_number: log_block_num,
            log_index_in_tx,
        };
        let end_block_num = log_block_num + 15;

        eth_events_tx
            .send((address, end_block_num, vec![eth_log.clone()]))
            .await
            .unwrap();

        // Executor should have received the action
        assert_eq!(
            executor_requested_action_rx.recv().await.unwrap(),
            bridge_action.digest()
        );
        let start = std::time::Instant::now();
        loop {
            let actions = store.get_all_pending_actions();
            if actions.is_empty() {
                if start.elapsed().as_secs() > 5 {
                    panic!("Timed out waiting for action to be written to WAL");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                continue;
            }
            assert_eq!(actions.len(), 1);
            let action = actions.get(&bridge_action.digest()).unwrap();
            assert_eq!(action, &bridge_action);
            assert_eq!(
                store.get_eth_event_cursors(&[address]).unwrap()[0].unwrap(),
                end_block_num,
            );
            break;
        }
    }

    #[tokio::test]
    async fn test_haneul_grpc_watcher_task() {
        let (
            haneul_grpc_events_tx,
            haneul_grpc_events_rx,
            _eth_events_tx,
            eth_events_rx,
            eth_monitor_tx,
            _eth_monitor_rx,
            haneul_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();
        // start orchestrator with gRPC
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(haneul_client),
            haneul_grpc_events_rx,
            eth_events_rx,
            store.clone(),
            eth_monitor_tx,
            metrics,
        )
        .run_with_grpc(executor)
        .await;

        let identifier = Identifier::from_str("test_haneul_grpc_watcher_task").unwrap();
        let (haneul_event, mut bridge_action) = get_test_haneul_event_and_action(identifier);
        bridge_action = bridge_action.update_to_token_transfer();

        // Convert HaneulEvent to HaneulBridgeEvent
        let bridge_event = HaneulBridgeEvent::try_from_haneul_event(&haneul_event)
            .unwrap()
            .unwrap();

        let last_seq_num = 42u64;
        haneul_grpc_events_tx
            .send((last_seq_num, vec![bridge_event]))
            .await
            .unwrap();

        let start = std::time::Instant::now();
        // Executor should have received the action
        assert_eq!(
            executor_requested_action_rx.recv().await.unwrap(),
            bridge_action.digest()
        );
        loop {
            let actions = store.get_all_pending_actions();
            if actions.is_empty() {
                if start.elapsed().as_secs() > 5 {
                    panic!("Timed out waiting for action to be written to WAL");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                continue;
            }
            assert_eq!(actions.len(), 1);
            let action = actions.get(&bridge_action.digest()).unwrap();
            assert_eq!(action, &bridge_action);
            // Verify sequence number cursor was updated
            assert_eq!(
                store.get_haneul_sequence_number_cursor().unwrap().unwrap(),
                last_seq_num,
            );
            break;
        }
    }

    #[tokio::test]
    /// Test that when orchestrator starts with gRPC, all pending actions are sent to executor
    async fn test_resume_actions_in_pending_logs_with_grpc() {
        let (
            _haneul_grpc_events_tx,
            haneul_grpc_events_rx,
            _eth_events_tx,
            eth_events_rx,
            eth_monitor_tx,
            _eth_monitor_rx,
            haneul_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();

        let action1 = get_test_haneul_to_eth_bridge_action(
            None,
            Some(0),
            Some(99),
            Some(10000),
            None,
            None,
            None,
        );

        let action2 = get_test_eth_to_haneul_bridge_action(None, None, None, None);
        store
            .insert_pending_actions(&[action1.clone(), action2.clone()])
            .unwrap();

        // start orchestrator with gRPC
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(haneul_client),
            haneul_grpc_events_rx,
            eth_events_rx,
            store.clone(),
            eth_monitor_tx,
            metrics,
        )
        .run_with_grpc(executor)
        .await;

        // Executor should have received the action
        let mut digests = std::collections::HashSet::new();
        digests.insert(executor_requested_action_rx.recv().await.unwrap());
        digests.insert(executor_requested_action_rx.recv().await.unwrap());
        assert!(digests.contains(&action1.digest()));
        assert!(digests.contains(&action2.digest()));
        assert_eq!(digests.len(), 2);
    }

    #[tokio::test]
    /// Test that when orchestrator starts, all pending actions are sent to executor
    async fn test_resume_actions_in_pending_logs() {
        let (
            _haneul_grpc_events_tx,
            haneul_grpc_events_rx,
            _eth_events_tx,
            eth_events_rx,
            eth_monitor_tx,
            _eth_monitor_rx,
            haneul_client,
            store,
        ) = setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();

        let action1 = get_test_haneul_to_eth_bridge_action(
            None,
            Some(0),
            Some(99),
            Some(10000),
            None,
            None,
            None,
        );

        let action2 = get_test_eth_to_haneul_bridge_action(None, None, None, None);
        store
            .insert_pending_actions(&[action1.clone(), action2.clone()])
            .unwrap();

        // start orchestrator
        let registry = Registry::new();
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let _handles = BridgeOrchestrator::new(
            Arc::new(haneul_client),
            haneul_grpc_events_rx,
            eth_events_rx,
            store.clone(),
            eth_monitor_tx,
            metrics,
        )
        .run_with_grpc(executor)
        .await;

        // Executor should have received the action
        let mut digests = std::collections::HashSet::new();
        digests.insert(executor_requested_action_rx.recv().await.unwrap());
        digests.insert(executor_requested_action_rx.recv().await.unwrap());
        assert!(digests.contains(&action1.digest()));
        assert!(digests.contains(&action2.digest()));
        assert_eq!(digests.len(), 2);
    }

    #[allow(clippy::type_complexity)]
    fn setup() -> (
        haneullabs_metrics::metered_channel::Sender<(u64, Vec<HaneulBridgeEvent>)>,
        haneullabs_metrics::metered_channel::Receiver<(u64, Vec<HaneulBridgeEvent>)>,
        haneullabs_metrics::metered_channel::Sender<(EthAddress, u64, Vec<EthLog>)>,
        haneullabs_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
        haneullabs_metrics::metered_channel::Sender<EthBridgeEvent>,
        haneullabs_metrics::metered_channel::Receiver<EthBridgeEvent>,
        HaneulClient<HaneulMockClient>,
        Arc<BridgeOrchestratorTables>,
    ) {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        haneullabs_metrics::init_metrics(&registry);

        init_all_struct_tags();

        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let mock_client = HaneulMockClient::default();
        let haneul_client = HaneulClient::new_for_testing(mock_client.clone());

        let (eth_events_tx, eth_events_rx) = haneullabs_metrics::metered_channel::channel(
            100,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["unit_test_eth_events_queue"]),
        );

        let (haneul_grpc_events_tx, haneul_grpc_events_rx) =
            haneullabs_metrics::metered_channel::channel(
                100,
                &haneullabs_metrics::get_metrics()
                    .unwrap()
                    .channel_inflight
                    .with_label_values(&["unit_test_haneul_events_queue"]),
            );
        let (eth_monitor_tx, eth_monitor_rx) = haneullabs_metrics::metered_channel::channel(
            10000,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["eth_monitor_queue"]),
        );
        (
            haneul_grpc_events_tx,
            haneul_grpc_events_rx,
            eth_events_tx,
            eth_events_rx,
            eth_monitor_tx,
            eth_monitor_rx,
            haneul_client,
            store,
        )
    }

    /// A `BridgeActionExecutorTrait` implementation that only tracks the submitted actions.
    struct MockExecutor {
        requested_transactions_tx: tokio::sync::broadcast::Sender<BridgeActionDigest>,
    }

    impl MockExecutor {
        fn new() -> (Self, tokio::sync::broadcast::Receiver<BridgeActionDigest>) {
            let (tx, rx) = tokio::sync::broadcast::channel(100);
            (
                Self {
                    requested_transactions_tx: tx,
                },
                rx,
            )
        }
    }

    impl BridgeActionExecutorTrait for MockExecutor {
        fn run(
            self,
        ) -> (
            Vec<tokio::task::JoinHandle<()>>,
            haneullabs_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        ) {
            let (tx, mut rx) =
                haneullabs_metrics::metered_channel::channel::<BridgeActionExecutionWrapper>(
                    100,
                    &haneullabs_metrics::get_metrics()
                        .unwrap()
                        .channel_inflight
                        .with_label_values(&["unit_test_mock_executor"]),
                );

            let handles = tokio::spawn(async move {
                while let Some(action) = rx.recv().await {
                    self.requested_transactions_tx
                        .send(action.0.digest())
                        .unwrap();
                }
            });
            (vec![handles], tx)
        }
    }
}
