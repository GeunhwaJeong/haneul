// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! The HaneulSyncer module is responsible for synchronizing Events emitted
//! on Haneul blockchain from concerned modules of bridge package 0x9.
//!
//! There are two modes of operation:
//! - Event-based (legacy): Uses JSON-RPC to query events by module
//! - gRPC-based (new): Iterates over bridge records using LinkedTable iteration
//!
//! As of now, only the event-based mode is being used.

use crate::{
    error::BridgeResult,
    events::{EmittedHaneulToEthTokenBridgeV1, EmittedHaneulToEthTokenBridgeV2, HaneulBridgeEvent},
    haneul_client::{HaneulClient, HaneulClientInner},
    metrics::BridgeMetrics,
    retry_with_max_elapsed_time,
    types::BridgeAction,
};
use haneul_types::{Identifier, event::EventID};
use haneullabs_metrics::spawn_logged_monitored_task;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::Notify,
    task::JoinHandle,
    time::{self, Duration},
};

const HANEUL_EVENTS_CHANNEL_SIZE: usize = 1000;

/// Map from contract address to their start cursor (exclusive)
pub type HaneulTargetModules = HashMap<Identifier, Option<EventID>>;

pub type GrpcSyncedEvents = (u64, Vec<HaneulBridgeEvent>);

pub struct HaneulSyncer<C> {
    haneul_client: Arc<HaneulClient<C>>,
    // The last transaction that the syncer has fully processed.
    // Syncer will resume post this transaction (i.e. exclusive), when it starts.
    #[allow(unused)]
    cursors: HaneulTargetModules,
    metrics: Arc<BridgeMetrics>,
}

impl<C> HaneulSyncer<C>
where
    C: HaneulClientInner + 'static,
{
    pub fn new(
        haneul_client: Arc<HaneulClient<C>>,
        cursors: HaneulTargetModules,
        metrics: Arc<BridgeMetrics>,
    ) -> Self {
        Self {
            haneul_client,
            cursors,
            metrics,
        }
    }

    pub async fn run_grpc(
        self,
        source_chain_id: u8,
        next_sequence_number: u64,
        query_interval: Duration,
        batch_size: u64,
    ) -> BridgeResult<(
        Vec<JoinHandle<()>>,
        haneullabs_metrics::metered_channel::Receiver<GrpcSyncedEvents>,
    )> {
        let (events_tx, events_rx) = haneullabs_metrics::metered_channel::channel(
            HANEUL_EVENTS_CHANNEL_SIZE,
            &haneullabs_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["haneul_grpc_events_queue"]),
        );

        let task_handle = spawn_logged_monitored_task!(Self::run_grpc_listening_task(
            source_chain_id,
            next_sequence_number,
            events_tx,
            self.haneul_client.clone(),
            query_interval,
            batch_size,
            self.metrics.clone(),
        ));

        Ok((vec![task_handle], events_rx))
    }

    async fn run_grpc_listening_task(
        source_chain_id: u8,
        mut next_sequence_cursor: u64,
        events_sender: haneullabs_metrics::metered_channel::Sender<GrpcSyncedEvents>,
        haneul_client: Arc<HaneulClient<C>>,
        query_interval: Duration,
        batch_size: u64,
        metrics: Arc<BridgeMetrics>,
    ) {
        tracing::info!(
            source_chain_id,
            next_sequence_cursor,
            "Starting haneul grpc records listening task"
        );
        let mut interval = time::interval(query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        // Create a task to update metrics
        let notify = Arc::new(Notify::new());
        let notify_clone = notify.clone();
        let haneul_client_clone = haneul_client.clone();
        let chain_label = source_chain_id.to_string();
        let last_synced_haneul_checkpoints_metric = metrics
            .last_synced_haneul_checkpoints
            .with_label_values(&[&chain_label]);
        spawn_logged_monitored_task!(async move {
            loop {
                notify_clone.notified().await;
                let Ok(Ok(latest_checkpoint_sequence_number)) = retry_with_max_elapsed_time!(
                    haneul_client_clone.get_latest_checkpoint_sequence_number(),
                    Duration::from_secs(120)
                ) else {
                    tracing::error!(
                        "Failed to query latest checkpoint sequence number from haneul client after retry"
                    );
                    continue;
                };
                last_synced_haneul_checkpoints_metric.set(latest_checkpoint_sequence_number as i64);
            }
        });

        loop {
            interval.tick().await;
            let Ok(Ok(on_chain_next_sequence_index)) = retry_with_max_elapsed_time!(
                haneul_client.get_token_transfer_next_seq_number(source_chain_id),
                Duration::from_secs(120)
            ) else {
                tracing::error!(
                    source_chain_id,
                    "Failed to get next seq num from haneul client after retry"
                );
                continue;
            };

            // start querying from the next_sequence_cursor till on_chain_next_sequence_index in batches
            let start_index = next_sequence_cursor;
            if start_index >= on_chain_next_sequence_index {
                notify.notify_one();
                continue;
            }

            let end_index = std::cmp::min(
                start_index + batch_size - 1,
                on_chain_next_sequence_index - 1,
            );

            let Ok(Ok(records)) = retry_with_max_elapsed_time!(
                haneul_client.get_bridge_records_in_range(source_chain_id, start_index, end_index),
                Duration::from_secs(120)
            ) else {
                tracing::error!(
                    source_chain_id,
                    start_index,
                    end_index,
                    "Failed to get records from haneul client after retry"
                );
                continue;
            };

            let len = records.len();
            if len != 0 {
                let mut events = Vec::with_capacity(len);
                let mut batch_last_sequence_index = start_index;

                for (seq_index, record) in records {
                    let event = match Self::bridge_record_to_event(&record, source_chain_id) {
                        Ok(event) => event,
                        Err(e) => {
                            tracing::error!(
                                source_chain_id,
                                seq_index,
                                "Failed to convert record to event: {:?}",
                                e
                            );
                            continue;
                        }
                    };

                    events.push(event);
                    batch_last_sequence_index = seq_index;
                }

                if !events.is_empty() {
                    events_sender
                        .send((batch_last_sequence_index + 1, events))
                        .await
                        .expect("Bridge events channel receiver is closed");

                    next_sequence_cursor = batch_last_sequence_index + 1;
                    tracing::info!(
                        source_chain_id,
                        last_processed_seq = batch_last_sequence_index,
                        next_sequence_cursor,
                        "Processed {len} bridge records"
                    );
                }
            }

            if end_index >= on_chain_next_sequence_index - 1 {
                // we have processed all records up to the latest checkpoint
                // so we can update the latest checkpoint metric
                notify.notify_one();
            }
        }
    }

    fn bridge_record_to_event(
        record: &haneul_types::bridge::MoveTypeBridgeRecord,
        source_chain_id: u8,
    ) -> Result<HaneulBridgeEvent, crate::error::BridgeError> {
        let action = BridgeAction::try_from_bridge_record(record)?;

        match action {
            BridgeAction::HaneulToEthTokenTransfer(transfer) => Ok(
                HaneulBridgeEvent::HaneulToEthTokenBridgeV1(EmittedHaneulToEthTokenBridgeV1 {
                    nonce: transfer.nonce,
                    haneul_chain_id: transfer.haneul_chain_id,
                    eth_chain_id: transfer.eth_chain_id,
                    haneul_address: transfer.haneul_address,
                    eth_address: transfer.eth_address,
                    token_id: transfer.token_id,
                    amount_haneul_adjusted: transfer.amount_adjusted,
                }),
            ),
            BridgeAction::HaneulToEthTokenTransferV2(transfer) => Ok(
                HaneulBridgeEvent::HaneulToEthTokenBridgeV2(EmittedHaneulToEthTokenBridgeV2 {
                    nonce: transfer.nonce,
                    haneul_chain_id: transfer.haneul_chain_id,
                    eth_chain_id: transfer.eth_chain_id,
                    haneul_address: transfer.haneul_address,
                    eth_address: transfer.eth_address,
                    token_id: transfer.token_id,
                    amount_haneul_adjusted: transfer.amount_adjusted,
                    timestamp_ms: transfer.timestamp_ms,
                }),
            ),
            _ => Err(crate::error::BridgeError::Generic(format!(
                "Unexpected action type for source_chain_id {}: {:?}",
                source_chain_id, action
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{haneul_client::HaneulClient, haneul_mock_client::HaneulMockClient};
    use haneul_types::bridge::{BridgeChainId, MoveTypeBridgeMessage, MoveTypeBridgeRecord};
    use prometheus::Registry;
    use tokio::time::timeout;

    async fn assert_no_more_events<T: std::fmt::Debug>(
        interval: Duration,
        events_rx: &mut haneullabs_metrics::metered_channel::Receiver<T>,
    ) {
        match timeout(interval * 2, events_rx.recv()).await {
            Err(_e) => (),
            other => panic!("Should have timed out, but got: {:?}", other),
        };
    }

    /// Creates a test bridge record with valid BCS-encoded payload
    fn create_test_bridge_record(
        seq_num: u64,
        source_chain: BridgeChainId,
        target_chain: BridgeChainId,
        amount: u64,
    ) -> MoveTypeBridgeRecord {
        // Create the payload struct matching HaneulToEthOnChainBcsPayload
        #[derive(serde::Serialize)]
        struct TestPayload {
            haneul_address: Vec<u8>,
            target_chain: u8,
            eth_address: Vec<u8>,
            token_type: u8,
            amount: [u8; 8],
        }

        let payload = TestPayload {
            haneul_address: vec![0u8; 32], // 32-byte HaneulAddress
            target_chain: target_chain as u8,
            eth_address: vec![0u8; 20], // 20-byte EthAddress
            token_type: 1,              // HANEUL token
            amount: amount.to_be_bytes(),
        };

        let payload_bytes = bcs::to_bytes(&payload).unwrap();

        MoveTypeBridgeRecord {
            message: MoveTypeBridgeMessage {
                message_type: 0, // TokenTransfer
                message_version: 1,
                seq_num,
                source_chain: source_chain as u8,
                payload: payload_bytes,
            },
            verified_signatures: None,
            claimed: false,
        }
    }

    #[tokio::test]
    async fn test_haneul_syncer_grpc_basic() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        haneullabs_metrics::init_metrics(&registry);
        let metrics = Arc::new(BridgeMetrics::new(&registry));
        let mock = HaneulMockClient::default();
        let client = Arc::new(HaneulClient::new_for_testing(mock.clone()));

        let source_chain_id = BridgeChainId::HaneulCustom as u8;
        let target_modules = HashMap::new(); // Not used for gRPC mode

        let interval = Duration::from_millis(200);
        let batch_size = 10;
        let next_sequence_number = 0;

        // Initially, no records on chain
        mock.set_next_seq_num(source_chain_id, 0);

        let (_handles, mut events_rx) =
            HaneulSyncer::new(client.clone(), target_modules.clone(), metrics.clone())
                .run_grpc(source_chain_id, next_sequence_number, interval, batch_size)
                .await
                .unwrap();

        // Initially there are no records
        assert_no_more_events(interval, &mut events_rx).await;

        mock.set_latest_checkpoint_sequence_number(1000);

        // Add some bridge records
        let record_0 = create_test_bridge_record(
            0,
            BridgeChainId::HaneulCustom,
            BridgeChainId::EthCustom,
            1000,
        );
        let record_1 = create_test_bridge_record(
            1,
            BridgeChainId::HaneulCustom,
            BridgeChainId::EthCustom,
            2000,
        );

        mock.add_bridge_record(source_chain_id, 0, record_0);
        mock.add_bridge_record(source_chain_id, 1, record_1);
        mock.set_next_seq_num(source_chain_id, 2); // 2 records available (0 and 1)

        let (next_cursor, received_events) = events_rx.recv().await.unwrap();
        assert_eq!(received_events.len(), 2);
        assert_eq!(next_cursor, 2); // Next sequence number to process

        match &received_events[0] {
            HaneulBridgeEvent::HaneulToEthTokenBridgeV1(event) => {
                assert_eq!(event.nonce, 0);
                assert_eq!(event.haneul_chain_id, BridgeChainId::HaneulCustom);
                assert_eq!(event.eth_chain_id, BridgeChainId::EthCustom);
                assert_eq!(event.amount_haneul_adjusted, 1000);
            }
            _ => panic!("Expected HaneulToEthTokenBridgeV1 event"),
        }
        match &received_events[1] {
            HaneulBridgeEvent::HaneulToEthTokenBridgeV1(event) => {
                assert_eq!(event.nonce, 1);
                assert_eq!(event.amount_haneul_adjusted, 2000);
            }
            _ => panic!("Expected HaneulToEthTokenBridgeV1 event"),
        }

        // No more events should be received
        assert_no_more_events(interval, &mut events_rx).await;
        assert_eq!(
            metrics
                .last_synced_haneul_checkpoints
                .get_metric_with_label_values(&[&source_chain_id.to_string()])
                .unwrap()
                .get(),
            1000
        );

        Ok(())
    }
}
