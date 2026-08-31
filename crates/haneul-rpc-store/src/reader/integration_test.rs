// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Integration test for the reader-trait stack. Seeds a small set
//! of objects, checkpoints, and transactions directly through
//! [`RpcStoreSchema`] (bypassing the indexer pipelines, which have
//! their own per-pipeline tests), wraps the database in a
//! [`RpcStoreReader`], and exercises representative entry points
//! across [`ObjectStore`], [`ReadStore`], [`RpcStateReader`], and
//! [`RpcIndexes`].
//!
//! The goal is to prove the trait impls compose correctly through
//! a single handle — not to re-cover the per-method scenarios
//! that the sibling unit tests already pin down.

use std::sync::Arc;

use haneul_consistent_store::Db;
use haneul_consistent_store::DbOptions;
use haneul_types::base_types::HaneulAddress;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::TransactionDigest;
use haneul_types::crypto::AggregateAuthoritySignature;
use haneul_types::crypto::AuthorityStrongQuorumSignInfo;
use haneul_types::digests::CheckpointContentsDigest;
use haneul_types::gas::GasCostSummary;
use haneul_types::messages_checkpoint::CheckpointSummary;
use haneul_types::object::Object;
use haneul_types::storage::ObjectStore;
use haneul_types::storage::ReadStore;
use haneul_types::storage::RpcStateReader;

use crate::RpcStoreSchema;
use crate::reader::RpcStoreReader;
use crate::schema::checkpoint_summary;
use crate::schema::objects;
use crate::schema::primitives::U64Be;

fn build_summary(seq: u64) -> CheckpointSummary {
    CheckpointSummary {
        epoch: 0,
        sequence_number: seq,
        network_total_transactions: 0,
        content_digest: CheckpointContentsDigest::new([seq as u8; 32]),
        previous_digest: None,
        epoch_rolling_gas_cost_summary: GasCostSummary::default(),
        timestamp_ms: 0,
        checkpoint_commitments: vec![],
        end_of_epoch_data: None,
        version_specific_data: vec![],
    }
}

fn dummy_signature() -> AuthorityStrongQuorumSignInfo {
    AuthorityStrongQuorumSignInfo {
        epoch: 0,
        signature: AggregateAuthoritySignature::default(),
        signers_map: roaring::RoaringBitmap::new(),
    }
}

#[test]
fn reader_satisfies_rpc_state_reader_trait() {
    // Static-only: confirms `Arc<RpcStoreReader>` coerces to
    // `Arc<dyn RpcStateReader>` (which is what haneul-rpc-api takes).
    fn assert_dyn(_: Arc<dyn RpcStateReader>) {}
    let dir = tempfile::tempdir().unwrap();
    let (db, schema) = Db::open::<RpcStoreSchema>(dir.path(), DbOptions::default()).unwrap();
    let reader = Arc::new(RpcStoreReader::new(db, Arc::new(schema)));
    assert_dyn(reader);
}

#[test]
fn integration_reads_objects_and_checkpoint_via_trait_surface() {
    let dir = tempfile::tempdir().unwrap();
    let (db, schema) = Db::open::<RpcStoreSchema>(dir.path(), DbOptions::default()).unwrap();

    // Seed a live object and a checkpoint header. The pipelines'
    // own tests already exercise the per-CF write logic; here we
    // just need a few rows to read back through the trait surface.
    let object =
        Object::with_id_owner_for_testing(ObjectID::from_single_byte(1), HaneulAddress::ZERO);
    let summary = build_summary(0);

    let mut batch = db.batch();
    batch
        .put(
            &schema.objects,
            &objects::Key {
                id: object.id(),
                version: object.version(),
            },
            &objects::store(&object),
        )
        .unwrap();
    batch
        .put(
            &schema.checkpoint_summary,
            &U64Be(0),
            &checkpoint_summary::store(&summary, &dummy_signature()),
        )
        .unwrap();
    batch.commit().unwrap();

    let reader = RpcStoreReader::new(db, Arc::new(schema));

    // ObjectStore.
    let got = reader.get_object(&object.id()).expect("object present");
    assert_eq!(got, object);

    // ReadStore — latest checkpoint should be the one we just
    // seeded.
    let latest = reader.get_latest_checkpoint().expect("checkpoint present");
    assert_eq!(latest.sequence_number(), &0);

    // RpcStateReader — without any chain id recorded the call
    // surfaces a missing-data error; that's the expected shape for
    // a freshly opened store that hasn't observed a checkpoint
    // through a pipeline yet.
    assert!(reader.get_chain_identifier().is_err());

    // RpcIndexes through the rollup's `indexes()`.
    let indexes = reader.indexes().expect("indexes present");
    let owned: Vec<_> = indexes
        .owned_objects_iter(HaneulAddress::ZERO, None, None)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    // No object_by_owner row was seeded — the index pipeline
    // would have written it, but this test bypasses pipelines.
    // The point here is that the iterator returns successfully.
    assert!(owned.is_empty());

    // Unrelated read returns missing (no transaction seeded).
    let missing_tx = reader.get_transaction(&TransactionDigest::new([7; 32]));
    assert!(missing_tx.is_none());
}

#[test]
fn integration_reads_against_a_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let (db, schema) = Db::open::<RpcStoreSchema>(dir.path(), DbOptions::default()).unwrap();

    let mut batch = db.batch();
    batch
        .put(
            &schema.checkpoint_summary,
            &U64Be(0),
            &checkpoint_summary::store(&build_summary(0), &dummy_signature()),
        )
        .unwrap();
    batch.commit().unwrap();

    // Take a snapshot at checkpoint 0 *before* adding checkpoint 1
    // so the snapshot-bound reader observes only the first.
    db.take_snapshot(haneul_consistent_store::Watermark::for_checkpoint(0));

    let mut batch = db.batch();
    batch
        .put(
            &schema.checkpoint_summary,
            &U64Be(1),
            &checkpoint_summary::store(&build_summary(1), &dummy_signature()),
        )
        .unwrap();
    batch.commit().unwrap();

    let snap = db.at_snapshot(0).expect("snapshot retained");
    let tip_reader = RpcStoreReader::new(db, Arc::new(schema));
    let snap_reader = tip_reader.at_snapshot(&snap);

    // Tip sees checkpoint 1.
    assert_eq!(
        tip_reader
            .get_latest_checkpoint()
            .unwrap()
            .sequence_number(),
        &1
    );
    // Snapshot sees only checkpoint 0.
    assert_eq!(
        snap_reader
            .get_latest_checkpoint()
            .unwrap()
            .sequence_number(),
        &0
    );
}
