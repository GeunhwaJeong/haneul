// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0
use crate::handlers::{
    BRIDGE, TOKEN_DEPOSITED_EVENT, TOKEN_DEPOSITED_EVENT_V2, TOKEN_TRANSFER_APPROVED,
    TOKEN_TRANSFER_CLAIMED, is_bridge_txn,
};
use crate::metrics::BridgeIndexerMetrics;
use crate::struct_tag;
use async_trait::async_trait;
use diesel_async::RunQueryDsl;
use haneul_bridge::events::{
    MoveTokenDepositedEvent, MoveTokenDepositedEventV2, MoveTokenTransferApproved,
    MoveTokenTransferClaimed,
};
use haneul_bridge_schema::models::{BridgeDataSource, TokenTransfer, TokenTransferStatus};
use haneul_bridge_schema::schema::token_transfer;
use haneul_indexer_alt_framework::pipeline::Processor;
use haneul_indexer_alt_framework::postgres::Connection;
use haneul_indexer_alt_framework::postgres::handler::Handler;
use haneul_indexer_alt_framework::types::BRIDGE_ADDRESS;
use haneul_indexer_alt_framework::types::effects::TransactionEffectsAPI;
use haneul_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use haneul_indexer_alt_framework::types::transaction::TransactionDataAPI;
use move_core_types::language_storage::StructTag;
use std::sync::Arc;
use tracing::info;

pub struct TokenTransferHandler {
    deposited_event_type: StructTag,
    deposited_event_v2_type: StructTag,
    approved_event_type: StructTag,
    claimed_event_type: StructTag,
    metrics: Arc<BridgeIndexerMetrics>,
}

impl TokenTransferHandler {
    pub fn new(metrics: Arc<BridgeIndexerMetrics>) -> Self {
        Self {
            deposited_event_type: struct_tag!(BRIDGE_ADDRESS, BRIDGE, TOKEN_DEPOSITED_EVENT),
            deposited_event_v2_type: struct_tag!(BRIDGE_ADDRESS, BRIDGE, TOKEN_DEPOSITED_EVENT_V2),
            approved_event_type: struct_tag!(BRIDGE_ADDRESS, BRIDGE, TOKEN_TRANSFER_APPROVED),
            claimed_event_type: struct_tag!(BRIDGE_ADDRESS, BRIDGE, TOKEN_TRANSFER_CLAIMED),
            metrics,
        }
    }
}

impl Default for TokenTransferHandler {
    fn default() -> Self {
        // For compatibility with existing code that doesn't pass metrics
        use prometheus::Registry;
        let registry = Registry::new();
        let metrics = BridgeIndexerMetrics::new(&registry);
        Self::new(metrics)
    }
}

#[async_trait]
impl Processor for TokenTransferHandler {
    const NAME: &'static str = "token_transfer";
    type Value = TokenTransfer;

    async fn process(
        &self,
        checkpoint: &Arc<Checkpoint>,
    ) -> Result<Vec<Self::Value>, anyhow::Error> {
        let timestamp_ms = checkpoint.summary.timestamp_ms as i64;
        let block_height = checkpoint.summary.sequence_number as i64;

        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !is_bridge_txn(tx) {
                continue;
            }
            for ev in tx.events.iter().flat_map(|e| &e.data) {
                let (chain_id, nonce, status) = if self.deposited_event_type == ev.type_ {
                    info!("Observed Haneul Deposit {:?}", ev);
                    let event: MoveTokenDepositedEvent = bcs::from_bytes(&ev.contents)?;

                    // Bridge-specific metrics for token deposits
                    self.metrics
                        .bridge_events_total
                        .with_label_values(&["token_deposited", "haneul"])
                        .inc();
                    self.metrics
                        .token_transfers_total
                        .with_label_values(&[
                            "haneul_to_eth",
                            "deposited",
                            &event.token_type.to_string(),
                        ])
                        .inc();
                    self.metrics
                        .token_transfer_gas_used
                        .with_label_values(&["haneul_to_eth", "true"])
                        .inc_by(tx.effects.gas_cost_summary().net_gas_usage() as u64);

                    (
                        event.source_chain,
                        event.seq_num,
                        TokenTransferStatus::Deposited,
                    )
                } else if self.deposited_event_v2_type == ev.type_ {
                    info!("Observed Haneul V2 Deposit {:?}", ev);
                    let event: MoveTokenDepositedEventV2 = bcs::from_bytes(&ev.contents)?;

                    // Bridge-specific metrics for V2 token deposits
                    self.metrics
                        .bridge_events_total
                        .with_label_values(&["token_deposited_v2", "haneul"])
                        .inc();
                    self.metrics
                        .token_transfers_total
                        .with_label_values(&[
                            "haneul_to_eth",
                            "deposited",
                            &event.token_type.to_string(),
                        ])
                        .inc();
                    self.metrics
                        .token_transfer_gas_used
                        .with_label_values(&["haneul_to_eth", "true"])
                        .inc_by(tx.effects.gas_cost_summary().net_gas_usage() as u64);

                    (
                        event.source_chain,
                        event.seq_num,
                        TokenTransferStatus::Deposited,
                    )
                } else if self.approved_event_type == ev.type_ {
                    info!("Observed Haneul Approval {:?}", ev);
                    let event: MoveTokenTransferApproved = bcs::from_bytes(&ev.contents)?;

                    // Bridge committee approval metrics
                    self.metrics
                        .bridge_events_total
                        .with_label_values(&["transfer_approved", "haneul"])
                        .inc();
                    self.metrics
                        .token_transfers_total
                        .with_label_values(&["eth_to_haneul", "approved", "unknown"])
                        .inc();

                    (
                        event.message_key.source_chain,
                        event.message_key.bridge_seq_num,
                        TokenTransferStatus::Approved,
                    )
                } else if self.claimed_event_type == ev.type_ {
                    info!("Observed Haneul Claim {:?}", ev);
                    let event: MoveTokenTransferClaimed = bcs::from_bytes(&ev.contents)?;

                    // Bridge transfer completion metrics
                    self.metrics
                        .bridge_events_total
                        .with_label_values(&["transfer_claimed", "haneul"])
                        .inc();
                    self.metrics
                        .token_transfers_total
                        .with_label_values(&["eth_to_haneul", "claimed", "unknown"])
                        .inc();

                    (
                        event.message_key.source_chain,
                        event.message_key.bridge_seq_num,
                        TokenTransferStatus::Claimed,
                    )
                } else {
                    return Ok(results);
                };

                results.push(TokenTransfer {
                    chain_id: chain_id as i32,
                    nonce: nonce as i64,
                    block_height,
                    timestamp_ms,
                    status,
                    data_source: BridgeDataSource::HANEUL,
                    is_finalized: true,
                    txn_hash: tx.transaction.digest().inner().to_vec(),
                    txn_sender: tx.transaction.sender().to_vec(),
                    gas_usage: tx.effects.gas_cost_summary().net_gas_usage(),
                });
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Handler for TokenTransferHandler {
    async fn commit<'a>(
        values: &[Self::Value],
        conn: &mut Connection<'a>,
    ) -> anyhow::Result<usize> {
        Ok(diesel::insert_into(token_transfer::table)
            .values(values)
            .on_conflict_do_nothing()
            .execute(conn)
            .await?)
    }
}
