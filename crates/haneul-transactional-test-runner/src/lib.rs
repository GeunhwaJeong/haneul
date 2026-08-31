// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! This module contains the transactional test runner instantiation for the Haneul adapter

#[cfg(feature = "testing")]
pub mod args;
#[cfg(feature = "testing")]
pub mod cursor;
#[cfg(feature = "testing")]
pub mod offchain_state;
#[cfg(feature = "testing")]
pub mod programmable_transaction_test_parser;
#[cfg(feature = "testing")]
mod simulator_persisted_store;
#[cfg(feature = "testing")]
pub mod test_adapter;

#[cfg(feature = "testing")]
pub use move_transactional_test_runner::framework::{
    create_adapter, run_tasks_with_adapter, run_test_impl,
};

#[cfg(feature = "testing")]
mod testing_imports {
    pub use super::simulator_persisted_store::PersistedStore;
    pub use super::test_adapter::{HaneulTestAdapter, PRE_COMPILED};
    pub use haneul_core::authority::AuthorityState;
    pub use haneul_core::authority::authority_per_epoch_store::CertLockGuard;
    pub use haneul_core::authority::authority_test_utils::submit_and_execute_with_error;
    pub use haneul_core::authority::shared_object_version_manager::AssignedVersions;
    pub use haneul_json_rpc::authority_state::StateRead;
    pub use haneul_json_rpc_types::EventFilter;
    pub use haneul_json_rpc_types::{DevInspectResults, DryRunTransactionBlockResponse};
    pub use haneul_storage::key_value_store::TransactionKeyValueStore;
    pub use haneul_types::base_types::HaneulAddress;
    pub use haneul_types::base_types::ObjectID;
    pub use haneul_types::base_types::VersionNumber;
    pub use haneul_types::committee::EpochId;
    pub use haneul_types::digests::TransactionDigest;
    pub use haneul_types::effects::TransactionEffects;
    pub use haneul_types::effects::TransactionEvents;
    pub use haneul_types::error::ExecutionError;
    pub use haneul_types::error::HaneulErrorKind;
    pub use haneul_types::error::HaneulResult;
    pub use haneul_types::event::Event;
    pub use haneul_types::executable_transaction::{
        ExecutableTransaction, VerifiedExecutableTransaction,
    };
    pub use haneul_types::haneul_system_state::HaneulSystemStateTrait;
    pub use haneul_types::haneul_system_state::epoch_start_haneul_system_state::EpochStartSystemStateTrait;
    pub use haneul_types::messages_checkpoint::CheckpointContentsDigest;
    pub use haneul_types::messages_checkpoint::VerifiedCheckpoint;
    pub use haneul_types::object::Object;
    pub use haneul_types::storage::ObjectStore;
    pub use haneul_types::storage::ReadStore;
    pub use haneul_types::transaction::Transaction;
    pub use haneul_types::transaction::TransactionKind;
    pub use haneul_types::transaction::{InputObjects, TransactionData};
    pub use rand::rngs::StdRng;
    pub use simulacrum::AdvanceEpochConfig;
    pub use simulacrum::Simulacrum;
    pub use simulacrum::SimulatorStore;
    pub use std::path::Path;
    pub use std::sync::Arc;
}
#[cfg(feature = "testing")]
use testing_imports::*;

#[cfg(feature = "testing")]
#[cfg_attr(not(msim), tokio::main)]
#[cfg_attr(msim, msim::main)]
pub async fn run_test(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let (_guard, _filter_handle) = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();
    run_test_impl::<HaneulTestAdapter>(path, Some(std::sync::Arc::new(PRE_COMPILED.clone())), None)
        .await?;
    Ok(())
}

#[cfg(feature = "testing")]
pub struct ValidatorWithFullnode {
    pub validator: Arc<AuthorityState>,
    pub fullnode: Arc<AuthorityState>,
    pub kv_store: Arc<TransactionKeyValueStore>,
    pending_effects: Vec<TransactionEffects>,
    next_checkpoint_seq: u64,
}

#[cfg(feature = "testing")]
#[allow(unused_variables)]
/// TODO: better name?
#[async_trait::async_trait]
pub trait TransactionalAdapter: Send + Sync + ReadStore {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)>;

    async fn read_input_objects(
        &self,
        transaction: Transaction,
        assigned_versions: AssignedVersions,
    ) -> HaneulResult<InputObjects>;

    fn prepare_txn(
        &self,
        transaction: Transaction,
        input_objects: InputObjects,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)>;

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint>;

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects>;

    async fn advance_epoch(&mut self, config: AdvanceEpochConfig) -> anyhow::Result<()>;

    async fn request_gas(
        &mut self,
        address: HaneulAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects>;

    async fn dry_run_transaction_block(
        &self,
        transaction_block: TransactionData,
    ) -> HaneulResult<DryRunTransactionBlockResponse>;

    async fn dev_inspect_transaction_block(
        &self,
        sender: HaneulAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> HaneulResult<DevInspectResults>;

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> HaneulResult<Vec<Event>>;

    async fn get_active_validator_addresses(&self) -> HaneulResult<Vec<HaneulAddress>>;

    fn get_object(&self, object_id: &ObjectID) -> Option<Object>;
}

#[cfg(feature = "testing")]
#[async_trait::async_trait]
impl TransactionalAdapter for ValidatorWithFullnode {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        let is_consensus_tx = transaction.is_consensus_tx();
        let (_, effects, execution_error) = submit_and_execute_with_error(
            &self.validator,
            Some(&self.fullnode),
            transaction,
            is_consensus_tx,
        )
        .await?;
        let effects = effects.into_data();
        self.pending_effects.push(effects.clone());
        Ok((effects, execution_error))
    }

    async fn read_input_objects(
        &self,
        transaction: Transaction,
        assigned_versions: AssignedVersions,
    ) -> HaneulResult<InputObjects> {
        let tx = VerifiedExecutableTransaction::new_unchecked(
            ExecutableTransaction::new_from_data_and_sig(
                transaction.data().clone(),
                haneul_types::executable_transaction::CertificateProof::Checkpoint(0, 0),
            ),
        );

        let epoch_store = self.validator.load_epoch_store_one_call_per_task().clone();
        self.validator.read_objects_for_execution(
            &CertLockGuard::dummy_for_tests(),
            &tx,
            &assigned_versions,
            &epoch_store,
        )
    }

    fn prepare_txn(
        &self,
        transaction: Transaction,
        input_objects: InputObjects,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        let tx = VerifiedExecutableTransaction::new_unchecked(
            ExecutableTransaction::new_from_data_and_sig(
                transaction.data().clone(),
                haneul_types::executable_transaction::CertificateProof::Checkpoint(0, 0),
            ),
        );

        let epoch_store = self.validator.load_epoch_store_one_call_per_task().clone();
        let (transaction_outputs, error) =
            self.validator
                .prepare_certificate_for_benchmark(&tx, input_objects, &epoch_store)?;
        Ok((transaction_outputs.effects, error))
    }

    async fn dry_run_transaction_block(
        &self,
        transaction_block: TransactionData,
    ) -> HaneulResult<DryRunTransactionBlockResponse> {
        self.fullnode
            .dry_exec_transaction(transaction_block)
            .await
            .map(|result| result.0)
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender: HaneulAddress,
        transaction_kind: TransactionKind,
        gas_price: Option<u64>,
    ) -> HaneulResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(
                sender,
                transaction_kind,
                gas_price,
                None,
                None,
                None,
                None,
                None,
            )
            .await
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        limit: usize,
    ) -> HaneulResult<Vec<Event>> {
        Ok(self
            .validator
            .query_events(
                &self.kv_store,
                EventFilter::Transaction(*tx_digest),
                None,
                limit,
                false,
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|haneul_event| haneul_event.into())
            .collect())
    }

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint> {
        let checkpoint_seq = self.next_checkpoint_seq;
        self.next_checkpoint_seq += 1;
        let effects = std::mem::take(&mut self.pending_effects);
        if !effects.is_empty() {
            let replay_txns = self
                .validator
                .settle_accumulator_for_testing(&effects, Some(checkpoint_seq))
                .await;
            self.fullnode
                .replay_settlement_for_testing(&replay_txns)
                .await;
        }
        self.get_checkpoint_by_sequence_number(0)
            .ok_or_else(|| anyhow::anyhow!("No genesis checkpoint found"))
    }

    async fn advance_clock(
        &mut self,
        _duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("advance_clock not supported")
    }

    async fn advance_epoch(&mut self, _config: AdvanceEpochConfig) -> anyhow::Result<()> {
        self.validator.reconfigure_for_testing().await;
        self.fullnode.reconfigure_for_testing().await;
        Ok(())
    }

    async fn request_gas(
        &mut self,
        _address: HaneulAddress,
        _amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        unimplemented!("request_gas not supported")
    }

    async fn get_active_validator_addresses(&self) -> HaneulResult<Vec<HaneulAddress>> {
        Ok(self
            .fullnode
            .get_system_state()
            .map_err(|e| {
                HaneulErrorKind::HaneulSystemStateReadError(format!(
                    "Failed to get system state from fullnode: {}",
                    e
                ))
            })?
            .into_haneul_system_state_summary()
            .active_validators
            .iter()
            .map(|x| x.haneul_address)
            .collect::<Vec<_>>())
    }

    fn get_object(&self, object_id: &ObjectID) -> Option<Object> {
        self.validator.get_object_store().get_object(object_id)
    }
}

#[cfg(feature = "testing")]
impl ReadStore for ValidatorWithFullnode {
    fn get_committee(
        &self,
        _epoch: haneul_types::committee::EpochId,
    ) -> Option<Arc<haneul_types::committee::Committee>> {
        todo!()
    }

    fn get_latest_epoch_id(&self) -> haneul_types::storage::error::Result<EpochId> {
        Ok(self.validator.epoch_store_for_testing().epoch())
    }

    fn get_latest_checkpoint_sequence_number(
        &self,
    ) -> haneul_types::storage::error::Result<
        haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    > {
        Ok(self.next_checkpoint_seq.saturating_sub(1))
    }

    fn get_latest_checkpoint(&self) -> haneul_types::storage::error::Result<VerifiedCheckpoint> {
        let sequence_number = self
            .validator
            .get_latest_checkpoint_sequence_number()
            .unwrap();
        Ok(self
            .get_checkpoint_by_sequence_number(sequence_number)
            .unwrap())
    }

    fn get_highest_verified_checkpoint(
        &self,
    ) -> haneul_types::storage::error::Result<VerifiedCheckpoint> {
        todo!()
    }

    fn get_highest_synced_checkpoint(
        &self,
    ) -> haneul_types::storage::error::Result<VerifiedCheckpoint> {
        todo!()
    }

    fn get_lowest_available_checkpoint(
        &self,
    ) -> haneul_types::storage::error::Result<
        haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    > {
        todo!()
    }

    fn get_checkpoint_by_digest(
        &self,
        _digest: &haneul_types::messages_checkpoint::CheckpointDigest,
    ) -> Option<VerifiedCheckpoint> {
        todo!()
    }

    fn get_checkpoint_by_sequence_number(
        &self,
        sequence_number: haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> Option<VerifiedCheckpoint> {
        self.validator
            .get_checkpoint_store()
            .get_checkpoint_by_sequence_number(sequence_number)
            .expect("db error")
    }

    fn get_checkpoint_contents_by_digest(
        &self,
        digest: &CheckpointContentsDigest,
    ) -> Option<haneul_types::messages_checkpoint::CheckpointContents> {
        self.validator
            .get_checkpoint_store()
            .get_checkpoint_contents(digest)
            .expect("db error")
    }

    fn get_checkpoint_contents_by_sequence_number(
        &self,
        _sequence_number: haneul_types::messages_checkpoint::CheckpointSequenceNumber,
    ) -> Option<haneul_types::messages_checkpoint::CheckpointContents> {
        todo!()
    }

    fn get_transaction(
        &self,
        tx_digest: &TransactionDigest,
    ) -> Option<Arc<haneul_types::transaction::VerifiedTransaction>> {
        self.validator
            .get_transaction_cache_reader()
            .get_transaction_block(tx_digest)
    }

    fn get_transaction_effects(&self, tx_digest: &TransactionDigest) -> Option<TransactionEffects> {
        self.validator
            .get_transaction_cache_reader()
            .get_executed_effects(tx_digest)
    }

    fn get_events(&self, digest: &TransactionDigest) -> Option<TransactionEvents> {
        self.validator
            .get_transaction_cache_reader()
            .get_events(digest)
    }

    fn get_full_checkpoint_contents(
        &self,
        _sequence_number: Option<haneul_types::messages_checkpoint::CheckpointSequenceNumber>,
        _digest: &CheckpointContentsDigest,
    ) -> Option<haneul_types::messages_checkpoint::VersionedFullCheckpointContents> {
        todo!()
    }

    fn get_unchanged_loaded_runtime_objects(
        &self,
        _digest: &TransactionDigest,
    ) -> Option<Vec<haneul_types::storage::ObjectKey>> {
        None
    }

    fn get_transaction_checkpoint(
        &self,
        _digest: &TransactionDigest,
    ) -> Option<haneul_types::messages_checkpoint::CheckpointSequenceNumber> {
        None
    }
}

#[cfg(feature = "testing")]
impl ObjectStore for ValidatorWithFullnode {
    fn get_object(&self, object_id: &ObjectID) -> Option<Object> {
        self.validator.get_object_store().get_object(object_id)
    }

    fn get_object_by_key(&self, object_id: &ObjectID, version: VersionNumber) -> Option<Object> {
        self.validator
            .get_object_store()
            .get_object_by_key(object_id, version)
    }
}

#[cfg(feature = "testing")]
#[async_trait::async_trait]
impl TransactionalAdapter for Simulacrum<StdRng, PersistedStore> {
    async fn execute_txn(
        &mut self,
        transaction: Transaction,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        Ok(self.execute_transaction(transaction)?)
    }

    async fn read_input_objects(
        &self,
        _transaction: Transaction,
        _assigned_versions: AssignedVersions,
    ) -> HaneulResult<InputObjects> {
        unimplemented!("read_input_objects not supported in simulator mode")
    }

    fn prepare_txn(
        &self,
        _transaction: Transaction,
        _input_objects: InputObjects,
    ) -> anyhow::Result<(TransactionEffects, Option<ExecutionError>)> {
        unimplemented!("prepare_txn not supported in simulator mode")
    }

    async fn dev_inspect_transaction_block(
        &self,
        _sender: HaneulAddress,
        _transaction_kind: TransactionKind,
        _gas_price: Option<u64>,
    ) -> HaneulResult<DevInspectResults> {
        unimplemented!("dev_inspect_transaction_block not supported in simulator mode")
    }

    async fn dry_run_transaction_block(
        &self,
        _transaction_block: TransactionData,
    ) -> HaneulResult<DryRunTransactionBlockResponse> {
        unimplemented!("dry_run_transaction_block not supported in simulator mode")
    }

    async fn query_tx_events_asc(
        &self,
        tx_digest: &TransactionDigest,
        _limit: usize,
    ) -> HaneulResult<Vec<Event>> {
        Ok(self
            .store()
            .get_transaction_events(tx_digest)
            .map(|x| x.data)
            .unwrap_or_default())
    }

    async fn create_checkpoint(&mut self) -> anyhow::Result<VerifiedCheckpoint> {
        Ok(self.create_checkpoint())
    }

    async fn advance_clock(
        &mut self,
        duration: std::time::Duration,
    ) -> anyhow::Result<TransactionEffects> {
        Ok(self.advance_clock(duration))
    }

    async fn advance_epoch(&mut self, config: AdvanceEpochConfig) -> anyhow::Result<()> {
        self.advance_epoch(config);
        Ok(())
    }

    async fn request_gas(
        &mut self,
        address: HaneulAddress,
        amount: u64,
    ) -> anyhow::Result<TransactionEffects> {
        self.request_gas(address, amount)
    }

    async fn get_active_validator_addresses(&self) -> HaneulResult<Vec<HaneulAddress>> {
        // TODO: this is a hack to get the validator addresses. Currently using start state
        //       but we should have a better way to get this information after reconfig
        Ok(self.epoch_start_state().get_validator_addresses())
    }

    fn get_object(&self, object_id: &ObjectID) -> Option<Object> {
        ObjectStore::get_object(&self.store(), object_id)
    }
}
