// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::object_set::fetch_transaction_object_set;
use crate::ErrorReason;
use crate::RpcError;
use crate::RpcService;
use crate::TransactionNotFoundError;
use haneul_rpc::field::FieldMaskTree;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::merge::Merge;
use haneul_rpc::proto::google::rpc::bad_request::FieldViolation;
use haneul_rpc::proto::haneul::rpc::v2::BatchGetTransactionsRequest;
use haneul_rpc::proto::haneul::rpc::v2::BatchGetTransactionsResponse;
use haneul_rpc::proto::haneul::rpc::v2::ExecutedTransaction;
use haneul_rpc::proto::haneul::rpc::v2::GetTransactionRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetTransactionResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetTransactionResult;
use haneul_rpc::proto::haneul::rpc::v2::ObjectSet as ProtoObjectSet;
use haneul_rpc::proto::haneul::rpc::v2::Transaction;
use haneul_rpc::proto::haneul::rpc::v2::UserSignature;
use haneul_rpc::proto::timestamp_ms_to_proto;
use haneul_sdk_types::Digest;
use haneul_types::balance_change::derive_balance_changes_2;
use haneul_types::full_checkpoint_content::ObjectSet;
use prost_types::FieldMask;

pub const MAX_BATCH_REQUESTS: usize = 200;
pub const READ_MASK_DEFAULT: &str = crate::read_mask_defaults::TRANSACTION;

#[tracing::instrument(skip(service))]
pub fn get_transaction(
    service: &RpcService,
    request: GetTransactionRequest,
) -> Result<GetTransactionResponse, RpcError> {
    let transaction_digest = request
        .digest
        .ok_or_else(|| {
            FieldViolation::new("digest")
                .with_description("missing digest")
                .with_reason(ErrorReason::FieldMissing)
        })?
        .parse::<Digest>()
        .map_err(|e| {
            FieldViolation::new("digest")
                .with_description(format!("invalid digest: {e}"))
                .with_reason(ErrorReason::FieldInvalid)
        })?;

    let read_mask = {
        let read_mask = request
            .read_mask
            .unwrap_or_else(|| FieldMask::from_str(READ_MASK_DEFAULT));
        read_mask
            .validate::<ExecutedTransaction>()
            .map_err(|path| {
                FieldViolation::new("read_mask")
                    .with_description(format!("invalid read_mask path: {path}"))
                    .with_reason(ErrorReason::FieldInvalid)
            })?;
        FieldMaskTree::from(read_mask)
    };

    let Some(transaction_checkpoint) = service
        .reader
        .inner()
        .get_transaction_checkpoint(&transaction_digest.into())
    else {
        return Err(TransactionNotFoundError(transaction_digest).into());
    };

    let latest_checkpoint = service
        .reader
        .inner()
        .get_latest_checkpoint()?
        .sequence_number;
    let lowest_available_checkpoint = service.reader.get_lowest_available_checkpoint()?;

    if !(lowest_available_checkpoint..=latest_checkpoint).contains(&transaction_checkpoint) {
        return Err(TransactionNotFoundError(transaction_digest).into());
    }

    let transaction_read = service.reader.get_transaction_read(transaction_digest)?;
    let objects = fetch_transaction_object_set(&service.reader, &transaction_read, &read_mask)?;

    let transaction = render_executed_transaction(
        service,
        transaction_read,
        &objects,
        transaction_checkpoint,
        &read_mask,
    )?;

    Ok(GetTransactionResponse::new(transaction))
}

#[tracing::instrument(skip(service))]
pub fn batch_get_transactions(
    service: &RpcService,
    BatchGetTransactionsRequest {
        digests, read_mask, ..
    }: BatchGetTransactionsRequest,
) -> Result<BatchGetTransactionsResponse, RpcError> {
    let read_mask = {
        let read_mask = read_mask.unwrap_or_else(|| FieldMask::from_str(READ_MASK_DEFAULT));
        read_mask
            .validate::<ExecutedTransaction>()
            .map_err(|path| {
                FieldViolation::new("read_mask")
                    .with_description(format!("invalid read_mask path: {path}"))
                    .with_reason(ErrorReason::FieldInvalid)
            })?;
        FieldMaskTree::from(read_mask)
    };

    if digests.len() > MAX_BATCH_REQUESTS {
        return Err(RpcError::new(
            tonic::Code::InvalidArgument,
            format!("number of batch requests exceed limit of {MAX_BATCH_REQUESTS}"),
        ));
    }

    let latest_checkpoint = service
        .reader
        .inner()
        .get_latest_checkpoint()?
        .sequence_number;
    let lowest_available_checkpoint = service.reader.get_lowest_available_checkpoint()?;

    let transactions = digests
        .into_iter()
        .enumerate()
        .map(|(idx, digest)| -> Result<ExecutedTransaction, RpcError> {
            let digest: Digest = digest.parse().map_err(|e| {
                FieldViolation::new_at("digests", idx)
                    .with_description(format!("invalid digest: {e}"))
                    .with_reason(ErrorReason::FieldInvalid)
            })?;

            let Some(transaction_checkpoint) = service
                .reader
                .inner()
                .get_transaction_checkpoint(&digest.into())
            else {
                return Err(TransactionNotFoundError(digest).into());
            };

            if !(lowest_available_checkpoint..=latest_checkpoint).contains(&transaction_checkpoint)
            {
                return Err(TransactionNotFoundError(digest).into());
            }

            let transaction_read = service.reader.get_transaction_read(digest)?;
            let objects =
                fetch_transaction_object_set(&service.reader, &transaction_read, &read_mask)?;

            render_executed_transaction(
                service,
                transaction_read,
                &objects,
                transaction_checkpoint,
                &read_mask,
            )
        })
        .map(|result| match result {
            Ok(transaction) => GetTransactionResult::new_transaction(transaction),
            Err(error) => GetTransactionResult::new_error(error.into_status_proto()),
        })
        .collect();

    Ok(BatchGetTransactionsResponse::new(transactions))
}

pub(crate) fn render_executed_transaction(
    service: &RpcService,
    crate::reader::TransactionRead {
        digest,
        transaction,
        signatures,
        effects,
        events,
        timestamp_ms,
        unchanged_loaded_runtime_objects,
    }: crate::reader::TransactionRead,
    objects: &ObjectSet,
    checkpoint: u64,
    mask: &FieldMaskTree,
) -> Result<ExecutedTransaction, RpcError> {
    let mut message = ExecutedTransaction::default();

    if mask.contains(ExecutedTransaction::DIGEST_FIELD) {
        message.digest = Some(digest.to_string());
    }

    if let Some(submask) = mask.subtree(ExecutedTransaction::TRANSACTION_FIELD) {
        message.transaction = Some(Transaction::merge_from(&transaction, &submask));
    }

    if let Some(submask) = mask.subtree(ExecutedTransaction::SIGNATURES_FIELD) {
        message.signatures = signatures
            .into_iter()
            .map(|s| UserSignature::merge_from(&s, &submask))
            .collect();
    }

    let unchanged_loaded_runtime_objects = unchanged_loaded_runtime_objects.unwrap_or_default();

    if let Some(submask) = mask.subtree(ExecutedTransaction::EFFECTS_FIELD) {
        let effects = service.render_effects_to_proto(
            &effects,
            &unchanged_loaded_runtime_objects,
            objects,
            &submask,
        );

        message.effects = Some(effects);
    }

    if let Some(submask) = mask.subtree(ExecutedTransaction::EVENTS_FIELD) {
        // For historical transactions read from the ledger, packages should already be in the
        // backing store, so we pass an empty ObjectSet for overlay resolution.
        message.events = events
            .map(|events| service.render_events_to_proto(&events, &submask, &ObjectSet::default()));
    }

    if mask.contains(ExecutedTransaction::CHECKPOINT_FIELD) {
        message.set_checkpoint(checkpoint);
    }

    if mask.contains(ExecutedTransaction::TIMESTAMP_FIELD) {
        message.timestamp = timestamp_ms.map(timestamp_ms_to_proto);
    }

    if mask.contains(ExecutedTransaction::BALANCE_CHANGES_FIELD) {
        message.balance_changes = derive_balance_changes_2(&effects, objects)
            .into_iter()
            .map(Into::into)
            .collect();
    }

    if let Some(object_mask) = mask
        .subtree(ExecutedTransaction::OBJECTS_FIELD)
        .and_then(|submask| submask.subtree(ProtoObjectSet::OBJECTS_FIELD))
    {
        message.objects = Some(
            ProtoObjectSet::default().with_objects(
                objects
                    .iter()
                    .map(|object| service.render_object_to_proto(object, &object_mask, objects))
                    .collect(),
            ),
        );
    }

    Ok(message)
}
