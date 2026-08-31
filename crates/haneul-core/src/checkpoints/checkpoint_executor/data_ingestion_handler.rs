// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneullabs_common::ZipDebugEqIteratorExt;

use crate::checkpoints::checkpoint_executor::{CheckpointExecutionData, CheckpointTransactionData};
use crate::execution_cache::TransactionCacheRead;
use haneul_rpc::field::FieldMask;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::merge::Merge;
use haneul_rpc::proto::haneul::rpc;
use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::error::{HaneulErrorKind, HaneulResult};
use haneul_types::full_checkpoint_content::{Checkpoint, ExecutedTransaction, ObjectSet};
use haneul_types::storage::ObjectStore;
use prost::Message;
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

pub(crate) fn store_checkpoint_locally(
    path: impl AsRef<Path>,
    checkpoint: &Checkpoint,
) -> HaneulResult {
    let path = path.as_ref();
    let sequence_number = checkpoint.summary.sequence_number;

    std::fs::create_dir_all(path).map_err(|err| {
        HaneulErrorKind::FileIOError(format!(
            "failed to save full checkpoint content locally {:?}",
            err
        ))
    })?;

    let mask = FieldMask::from_paths([
        rpc::v2::Checkpoint::path_builder().sequence_number(),
        rpc::v2::Checkpoint::path_builder().summary().bcs().value(),
        rpc::v2::Checkpoint::path_builder().signature().finish(),
        rpc::v2::Checkpoint::path_builder().contents().bcs().value(),
        rpc::v2::Checkpoint::path_builder()
            .transactions()
            .transaction()
            .bcs()
            .value(),
        rpc::v2::Checkpoint::path_builder()
            .transactions()
            .effects()
            .bcs()
            .value(),
        rpc::v2::Checkpoint::path_builder()
            .transactions()
            .effects()
            .unchanged_loaded_runtime_objects()
            .finish(),
        rpc::v2::Checkpoint::path_builder()
            .transactions()
            .events()
            .bcs()
            .value(),
        rpc::v2::Checkpoint::path_builder()
            .objects()
            .objects()
            .bcs()
            .value(),
    ]);

    let proto_checkpoint = rpc::v2::Checkpoint::merge_from(checkpoint, &mask.into());
    let proto_bytes = proto_checkpoint.encode_to_vec();
    let compressed = zstd::encode_all(&proto_bytes[..], 3).map_err(|_| {
        HaneulErrorKind::TransactionSerializationError {
            error: "failed to compress checkpoint content".to_string(),
        }
    })?;

    let file_name = format!("{}.binpb.zst", sequence_number);
    std::fs::write(path.join(file_name), compressed).map_err(|_| {
        HaneulErrorKind::FileIOError("failed to save full checkpoint content locally".to_string())
    })?;

    Ok(())
}

pub(crate) fn load_checkpoint(
    ckpt_data: &CheckpointExecutionData,
    ckpt_tx_data: &CheckpointTransactionData,
    object_store: &dyn ObjectStore,
    transaction_cache_reader: &dyn TransactionCacheRead,
) -> HaneulResult<Checkpoint> {
    let event_tx_digests = ckpt_tx_data
        .effects
        .iter()
        .flat_map(|fx| fx.events_digest().map(|_| fx.transaction_digest()).copied())
        .collect::<Vec<_>>();

    let mut events = transaction_cache_reader
        .multi_get_events(&event_tx_digests)
        .into_iter()
        .zip_debug_eq(event_tx_digests)
        .map(|(maybe_event, tx_digest)| {
            maybe_event
                .ok_or(HaneulErrorKind::TransactionEventsNotFound { digest: tx_digest }.into())
                .map(|event| (tx_digest, event))
        })
        .collect::<HaneulResult<HashMap<_, _>>>()?;

    let mut transactions = Vec::with_capacity(ckpt_tx_data.transactions.len());
    for (tx, fx) in ckpt_tx_data
        .transactions
        .iter()
        .zip_debug_eq(ckpt_tx_data.effects.iter())
    {
        let events = fx.events_digest().map(|_event_digest| {
            events
                .remove(fx.transaction_digest())
                .expect("event was already checked to be present")
        });

        let transaction = ExecutedTransaction {
            transaction: tx.transaction_data().clone(),
            signatures: tx.tx_signatures().to_vec(),
            effects: fx.clone(),
            events,
            unchanged_loaded_runtime_objects: transaction_cache_reader
                .get_unchanged_loaded_runtime_objects(tx.digest())
                // We don't write empty sets to the DB to save space, so if this load went through
                // the writeback cache to the DB itself it wouldn't find an entry.
                .unwrap_or_default(),
        };
        transactions.push(transaction);
    }

    let object_set = {
        let refs = transactions
            .iter()
            .flat_map(|tx| {
                haneul_types::storage::get_transaction_object_set(
                    &tx.transaction,
                    &tx.effects,
                    &tx.unchanged_loaded_runtime_objects,
                )
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        let objects = object_store.multi_get_objects_by_key(&refs);

        let mut object_set = ObjectSet::default();
        for (idx, object) in objects.into_iter().enumerate() {
            object_set.insert(object.ok_or_else(|| {
                haneul_types::storage::error::Error::custom(format!(
                    "unabled to load object {:?}",
                    refs[idx]
                ))
            })?);
        }
        object_set
    };
    let checkpoint = Checkpoint {
        summary: ckpt_data.checkpoint.clone().into(),
        contents: ckpt_data.checkpoint_contents.clone(),
        transactions,
        object_set,
    };
    Ok(checkpoint)
}
