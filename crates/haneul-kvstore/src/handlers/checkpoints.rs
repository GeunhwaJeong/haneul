// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use haneul_indexer_alt_framework::pipeline::Processor;
use haneul_types::full_checkpoint_content::Checkpoint;

use crate::bigtable::proto::bigtable::v2::mutate_rows_request::Entry;
use crate::handlers::BigTableProcessor;
use crate::tables;

/// Pipeline that writes checkpoint summaries to the `checkpoints` table in BigTable.
/// Wrap with `BigTableHandler` for the full `concurrent::Handler` implementation.
pub struct CheckpointsPipeline;

#[async_trait::async_trait]
impl Processor for CheckpointsPipeline {
    const NAME: &'static str = "kvstore_checkpoints";
    type Value = Entry;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let summary = checkpoint.summary.data();
        let signatures = checkpoint.summary.auth_sig();
        let timestamp_ms = summary.timestamp_ms;

        let entry = tables::make_entry(
            tables::checkpoints::encode_key(summary.sequence_number),
            tables::checkpoints::encode(summary, signatures, &checkpoint.contents)?,
            Some(timestamp_ms),
        );

        Ok(vec![entry])
    }
}

impl BigTableProcessor for CheckpointsPipeline {
    const TABLE: &'static str = tables::checkpoints::NAME;
}
