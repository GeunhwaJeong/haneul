// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Shared test fixtures for the embedded rpc-store ingestion and
//! streaming clients: an in-memory [`ReadStore`] holding a handful of
//! pre-built full checkpoints.

use std::collections::BTreeMap;
use std::sync::Arc;

use haneul_types::base_types::ObjectID;
use haneul_types::base_types::TransactionDigest;
use haneul_types::base_types::VersionNumber;
use haneul_types::committee::Committee;
use haneul_types::committee::EpochId;
use haneul_types::digests::ChainIdentifier;
use haneul_types::digests::CheckpointDigest;
use haneul_types::effects::TransactionEffects;
use haneul_types::effects::TransactionEvents;
use haneul_types::full_checkpoint_content::Checkpoint;
use haneul_types::messages_checkpoint::CheckpointContents;
use haneul_types::messages_checkpoint::CheckpointContentsDigest;
use haneul_types::messages_checkpoint::CheckpointSequenceNumber;
use haneul_types::messages_checkpoint::VerifiedCheckpoint;
use haneul_types::messages_checkpoint::VersionedFullCheckpointContents;
use haneul_types::object::Object;
use haneul_types::storage::ObjectKey;
use haneul_types::storage::ObjectStore;
use haneul_types::storage::ReadStore;
use haneul_types::storage::error::Error as StorageError;
use haneul_types::storage::error::Result as StorageResult;
use haneul_types::test_checkpoint_data_builder::TestCheckpointBuilder;
use haneul_types::transaction::VerifiedTransaction;

const TEST_CHAIN_ID_BYTES: [u8; 32] = [7u8; 32];

/// A deterministic chain identifier for client tests.
pub(crate) fn test_chain_id() -> ChainIdentifier {
    CheckpointDigest::new(TEST_CHAIN_ID_BYTES).into()
}

/// Build a stand-alone full checkpoint at `sequence_number`.
pub(crate) fn checkpoint(sequence_number: u64) -> Arc<Checkpoint> {
    Arc::new(TestCheckpointBuilder::new(sequence_number).build_checkpoint())
}

/// In-memory [`ReadStore`] holding a handful of pre-built full
/// checkpoints. Only the methods the ingestion / streaming clients
/// exercise are implemented; the rest panic so a future change that
/// starts relying on them is caught loudly.
#[derive(Clone)]
pub(crate) struct MockReadStore {
    pub(crate) checkpoints: BTreeMap<CheckpointSequenceNumber, Checkpoint>,
    /// When set, `get_checkpoint_contents_by_digest` returns `None` for
    /// this checkpoint even though its summary is present, exercising the
    /// summary-without-contents NotFound path.
    pub(crate) drop_contents_for: Option<CheckpointSequenceNumber>,
}

/// Build a [`MockReadStore`] holding a full checkpoint for each of
/// `seqs`.
pub(crate) fn store_with(seqs: impl IntoIterator<Item = u64>) -> MockReadStore {
    let checkpoints = seqs
        .into_iter()
        .map(|seq| (seq, TestCheckpointBuilder::new(seq).build_checkpoint()))
        .collect();
    MockReadStore {
        checkpoints,
        drop_contents_for: None,
    }
}

impl ObjectStore for MockReadStore {
    fn get_object(&self, _: &ObjectID) -> Option<Object> {
        unimplemented!("the rpc-store clients never load objects directly")
    }

    fn get_object_by_key(&self, _: &ObjectID, _: VersionNumber) -> Option<Object> {
        unimplemented!("the rpc-store clients never load objects directly")
    }
}

impl ReadStore for MockReadStore {
    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: CheckpointSequenceNumber,
    ) -> Option<VerifiedCheckpoint> {
        self.checkpoints
            .get(&sequence_number)
            .map(|cp| VerifiedCheckpoint::new_unchecked(cp.summary.clone()))
    }

    fn get_checkpoint_contents_by_digest(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> Option<CheckpointContents> {
        self.checkpoints.values().find_map(|cp| {
            (cp.summary.content_digest == *digest
                && self.drop_contents_for != Some(*cp.summary.sequence_number()))
            .then(|| cp.contents.clone())
        })
    }

    fn get_checkpoint_data(
        &self,
        checkpoint: VerifiedCheckpoint,
        _contents: CheckpointContents,
    ) -> StorageResult<Checkpoint> {
        Ok(self.checkpoints[checkpoint.sequence_number()].clone())
    }

    fn get_latest_checkpoint(&self) -> StorageResult<VerifiedCheckpoint> {
        self.checkpoints
            .values()
            .next_back()
            .map(|cp| VerifiedCheckpoint::new_unchecked(cp.summary.clone()))
            .ok_or_else(|| StorageError::missing("no checkpoints"))
    }

    fn get_committee(&self, _: EpochId) -> Option<Arc<Committee>> {
        unimplemented!()
    }

    fn get_highest_verified_checkpoint(&self) -> StorageResult<VerifiedCheckpoint> {
        unimplemented!()
    }

    fn get_highest_synced_checkpoint(&self) -> StorageResult<VerifiedCheckpoint> {
        unimplemented!()
    }

    fn get_lowest_available_checkpoint(&self) -> StorageResult<CheckpointSequenceNumber> {
        Ok(0)
    }

    fn get_checkpoint_by_digest(&self, _: &CheckpointDigest) -> Option<VerifiedCheckpoint> {
        unimplemented!()
    }

    fn get_checkpoint_contents_by_sequence_number(
        &self,
        _: CheckpointSequenceNumber,
    ) -> Option<CheckpointContents> {
        unimplemented!()
    }

    fn get_transaction(&self, _: &TransactionDigest) -> Option<Arc<VerifiedTransaction>> {
        unimplemented!()
    }

    fn get_transaction_effects(&self, _: &TransactionDigest) -> Option<TransactionEffects> {
        unimplemented!()
    }

    fn get_events(&self, _: &TransactionDigest) -> Option<TransactionEvents> {
        unimplemented!()
    }

    fn get_unchanged_loaded_runtime_objects(
        &self,
        _: &TransactionDigest,
    ) -> Option<Vec<ObjectKey>> {
        unimplemented!()
    }

    fn get_transaction_checkpoint(
        &self,
        _: &TransactionDigest,
    ) -> Option<CheckpointSequenceNumber> {
        unimplemented!()
    }

    fn get_full_checkpoint_contents(
        &self,
        _: Option<CheckpointSequenceNumber>,
        _: &CheckpointContentsDigest,
    ) -> Option<VersionedFullCheckpointContents> {
        unimplemented!()
    }
}
