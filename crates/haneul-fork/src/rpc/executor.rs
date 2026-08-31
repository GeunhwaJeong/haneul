// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Adapter that exposes the forked-network's Simulacrum through the
//! `haneul_types::transaction_executor::TransactionExecutor` trait so that the
//! `TransactionExecutionService` gRPC endpoints served by `haneul-rpc-api` can drive it.

use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::info;

use haneul_types::effects::TransactionEffectsAPI;
use haneul_types::error::HaneulError;
use haneul_types::error::HaneulErrorKind;
use haneul_types::storage::get_transaction_input_objects;
use haneul_types::storage::get_transaction_output_objects;
use haneul_types::transaction::TransactionData;
use haneul_types::transaction_driver_types::EffectsFinalityInfo;
use haneul_types::transaction_driver_types::ExecuteTransactionRequestV3;
use haneul_types::transaction_driver_types::ExecuteTransactionResponseV3;
use haneul_types::transaction_driver_types::FinalizedEffects;
use haneul_types::transaction_driver_types::TransactionSubmissionError;
use haneul_types::transaction_executor::SimulateTransactionResult;
use haneul_types::transaction_executor::TransactionChecks;
use haneul_types::transaction_executor::TransactionExecutor;
use simulacrum::SimulatorStore;

use crate::context::Context;

/// `TransactionExecutor` implementation that runs transactions against the forked network's
/// Simulacrum. Empty-signature transactions explicitly request sender impersonation, while signed
/// transactions keep Simulacrum's normal user-signature verification. Each accepted execution is
/// sealed into a fresh Simulacrum checkpoint and published to checkpoint subscribers.
pub(crate) struct ForkedTransactionExecutor {
    context: Arc<Context>,
}

impl ForkedTransactionExecutor {
    pub(crate) fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl TransactionExecutor for ForkedTransactionExecutor {
    async fn execute_transaction(
        &self,
        request: ExecuteTransactionRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<ExecuteTransactionResponseV3, TransactionSubmissionError> {
        let ExecuteTransactionRequestV3 {
            transaction,
            include_events,
            include_input_objects,
            include_output_objects,
            include_auxiliary_data: _,
        } = request;

        // Execute under the serialized checkpoint path, then seal the
        // transaction into a fresh checkpoint so downstream reads see it as
        // finalized and subscribers are notified in sequence.
        let ((effects, exec_error), checkpoint_metadata) = self
            .context
            .try_run_with_new_checkpoint(|sim| {
                let (effects, exec_error) = sim
                    .execute_transaction_impersonating(transaction)
                    .map_err(into_submission_error)?;
                Ok((effects, exec_error))
            })
            .await?;
        let checkpoint_seq = checkpoint_metadata.sequence_number;

        let digest = *effects.transaction_digest();
        if let Some(err) = &exec_error {
            info!(%digest, checkpoint_seq, "forked transaction executed with error: {err:?}");
        } else {
            info!(%digest, checkpoint_seq, "forked transaction executed");
        }

        let events = if include_events && effects.events_digest().is_some() {
            let sim = self.context.simulacrum().read().await;
            sim.store().get_transaction_events(&digest)
        } else {
            None
        };

        // Input/output objects are resolved via the `ForkStore`, which is
        // the same `ObjectStore` the gRPC reader serves from — after
        // execution it holds the pre-execution input versions (from the
        // fork snapshot / sparse RPC-store saves) and the newly
        // written output versions.
        let sim = self.context.simulacrum().read().await;
        let object_store = sim.store();
        let input_objects = if include_input_objects {
            Some(
                get_transaction_input_objects(object_store, &effects).map_err(|e| {
                    TransactionSubmissionError::TransactionDriverInternalError(HaneulError::from(
                        format!("failed to resolve input objects for {digest}: {e}"),
                    ))
                })?,
            )
        } else {
            None
        };
        let output_objects = if include_output_objects {
            Some(
                get_transaction_output_objects(object_store, &effects).map_err(|e| {
                    TransactionSubmissionError::TransactionDriverInternalError(HaneulError::from(
                        format!("failed to resolve output objects for {digest}: {e}"),
                    ))
                })?,
            )
        } else {
            None
        };

        let executed_epoch = effects.executed_epoch();

        Ok(ExecuteTransactionResponseV3 {
            effects: FinalizedEffects {
                effects,
                // The forked network is single-node with no consensus; we
                // report the effects as executed within their embedded epoch.
                finality_info: EffectsFinalityInfo::Checkpointed(executed_epoch, checkpoint_seq),
            },
            events,
            input_objects,
            output_objects,
            auxiliary_data: None,
        })
    }

    fn simulate_transaction(
        &self,
        _transaction: TransactionData,
        _checks: TransactionChecks,
        _allow_mock_gas_coin: bool,
    ) -> Result<SimulateTransactionResult, HaneulError> {
        Err(HaneulErrorKind::Unknown(
            "simulate_transaction is not supported by the forked network yet".to_string(),
        )
        .into())
    }
}

fn into_submission_error(e: anyhow::Error) -> TransactionSubmissionError {
    match e.downcast::<HaneulError>() {
        Ok(haneul_error) if is_signature_error(&haneul_error) => {
            TransactionSubmissionError::InvalidUserSignature(haneul_error)
        }
        Ok(haneul_error) => {
            TransactionSubmissionError::TransactionDriverInternalError(haneul_error)
        }
        Err(other) => TransactionSubmissionError::TransactionDriverInternalError(
            HaneulError::from(format!("forked execution failed: {other}")),
        ),
    }
}

fn is_signature_error(e: &HaneulError) -> bool {
    matches!(
        &**e,
        HaneulErrorKind::InvalidSignature { .. }
            | HaneulErrorKind::SignerSignatureAbsent { .. }
            | HaneulErrorKind::SignerSignatureNumberMismatch { .. }
            | HaneulErrorKind::IncorrectSigner { .. }
    )
}
