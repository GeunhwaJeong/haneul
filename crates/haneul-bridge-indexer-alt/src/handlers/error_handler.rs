// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::handlers::is_bridge_txn;
use async_trait::async_trait;
use diesel_async::RunQueryDsl;
use haneul_bridge_schema::models::HaneulErrorTransactions;
use haneul_bridge_schema::schema::haneul_error_transactions;
use haneul_indexer_alt_framework::pipeline::Processor;
use haneul_indexer_alt_framework::postgres::Connection;
use haneul_indexer_alt_framework::postgres::handler::Handler;
use haneul_indexer_alt_framework::types::effects::TransactionEffectsAPI;
use haneul_indexer_alt_framework::types::execution_status::{ExecutionFailure, ExecutionStatus};
use haneul_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use haneul_indexer_alt_framework::types::transaction::TransactionDataAPI;
use std::sync::Arc;

pub struct ErrorTransactionHandler;

#[async_trait]
impl Processor for ErrorTransactionHandler {
    const NAME: &'static str = "error_transactions";
    type Value = HaneulErrorTransactions;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let timestamp_ms = checkpoint.summary.timestamp_ms as i64;
        let mut results = vec![];

        for tx in &checkpoint.transactions {
            if !is_bridge_txn(tx) {
                continue;
            }
            if let ExecutionStatus::Failure(ExecutionFailure { error, command }) =
                tx.effects.status()
            {
                results.push(HaneulErrorTransactions {
                    txn_digest: tx.transaction.digest().inner().to_vec(),
                    timestamp_ms,
                    failure_status: error.to_string(),
                    cmd_idx: command.map(|idx| idx as i64),
                    sender_address: tx.transaction.sender().to_vec(),
                })
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Handler for ErrorTransactionHandler {
    async fn commit<'a>(
        values: &[Self::Value],
        conn: &mut Connection<'a>,
    ) -> anyhow::Result<usize> {
        Ok(diesel::insert_into(haneul_error_transactions::table)
            .values(values)
            .on_conflict_do_nothing()
            .execute(conn)
            .await?)
    }
}
