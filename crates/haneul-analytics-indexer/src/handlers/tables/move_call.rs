// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use haneul_indexer_alt_framework::pipeline::Processor;
use haneul_types::base_types::EpochId;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::full_checkpoint_content::Checkpoint;
use haneul_types::transaction::TransactionDataAPI;

use crate::Row;
use crate::pipeline::Pipeline;
use crate::tables::MoveCallRow;

pub struct MoveCallProcessor;

impl Row for MoveCallRow {
    fn get_epoch(&self) -> EpochId {
        self.epoch
    }

    fn get_checkpoint(&self) -> u64 {
        self.checkpoint
    }
}

#[async_trait]
impl Processor for MoveCallProcessor {
    const NAME: &'static str = Pipeline::MoveCall.name();
    type Value = MoveCallRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.data().epoch;
        let checkpoint_seq = checkpoint.summary.data().sequence_number;
        let timestamp_ms = checkpoint.summary.data().timestamp_ms;

        let mut entries = Vec::new();

        for executed_tx in &checkpoint.transactions {
            let move_calls = executed_tx.transaction.move_calls();
            let transaction_digest = executed_tx.effects.transaction_digest().base58_encode();

            for (cmd_idx, package, module, function) in move_calls.iter() {
                let row = MoveCallRow {
                    transaction_digest: transaction_digest.clone(),
                    cmd_idx: *cmd_idx as u64,
                    checkpoint: checkpoint_seq,
                    epoch,
                    timestamp_ms,
                    package: package.to_string(),
                    module: module.to_string(),
                    function: function.to_string(),
                };
                entries.push(row);
            }
        }

        Ok(entries)
    }
}
