// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module haneul_system::haneul_system_state_inner {
    use haneul::balance::{Self, Balance};
    use haneul::haneul::HANEUL;
    use haneul::tx_context::TxContext;
    use haneul::bag::{Self, Bag};
    use haneul::table::{Self, Table};
    use haneul::object::ID;

    use haneul_system::validator::Validator;
    use haneul_system::validator_wrapper::ValidatorWrapper;

    const SYSTEM_STATE_VERSION_V1: u64 = 18446744073709551605;  // u64::MAX - 10
    const SYSTEM_STATE_VERSION_V2: u64 = 18446744073709551606;  // u64::MAX - 9

    const EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_KEY: u64 = 0;
    const EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_CHUNK_COUNT_KEY: u64 = 1;

    public struct ExecutionTimeObservationChunkKey has copy, drop, store {
        chunk_index: u64,
    }

    public struct SystemParameters has store {
        epoch_duration_ms: u64,
        extra_fields: Bag,
    }

    public struct ValidatorSet has store {
        active_validators: vector<Validator>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    public struct HaneulSystemStateInner has store {
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<HANEUL>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public struct HaneulSystemStateInnerV2 has store {
        new_dummy_field: u64,
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<HANEUL>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public(package) fun create(
        validators: vector<Validator>,
        storage_fund: Balance<HANEUL>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ): HaneulSystemStateInner {
        let validators = new_validator_set(validators, ctx);
        let system_state = HaneulSystemStateInner {
            epoch: 0,
            protocol_version,
            system_state_version: genesis_system_state_version(),
            validators,
            storage_fund,
            parameters: SystemParameters {
                epoch_duration_ms,
                extra_fields: bag::new(ctx),
            },
            reference_gas_price: 1,
            safe_mode: false,
            epoch_start_timestamp_ms,
            extra_fields: bag::new(ctx),
        };
        system_state
    }

    public(package) fun advance_epoch(
        self: &mut HaneulSystemStateInnerV2,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<HANEUL>,
        computation_reward: Balance<HANEUL>,
        storage_rebate_amount: u64,
        epoch_start_timestamp_ms: u64,
    ) : Balance<HANEUL> {
        self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;
        self.epoch = self.epoch + 1;
        assert!(new_epoch == self.epoch, 0);
        self.safe_mode = false;
        self.protocol_version = next_protocol_version;

        balance::join(&mut self.storage_fund, computation_reward);
        balance::join(&mut self.storage_fund, storage_reward);
        let storage_rebate = balance::split(&mut self.storage_fund, storage_rebate_amount);
        storage_rebate
    }

    public(package) fun protocol_version(self: &HaneulSystemStateInnerV2): u64 { self.protocol_version }
    public(package) fun system_state_version(self: &HaneulSystemStateInnerV2): u64 { self.system_state_version }
    public(package) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    fun new_validator_set(init_active_validators: vector<Validator>, ctx: &mut TxContext): ValidatorSet {
        ValidatorSet {
            active_validators: init_active_validators,
            inactive_validators: table::new(ctx),
            extra_fields: bag::new(ctx),
        }
    }

    public(package) fun v1_to_v2(v1: HaneulSystemStateInner): HaneulSystemStateInnerV2 {
        let HaneulSystemStateInner {
            epoch,
            protocol_version,
            system_state_version: old_system_state_version,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        } = v1;
        assert!(old_system_state_version == SYSTEM_STATE_VERSION_V1, 0);
        HaneulSystemStateInnerV2 {
            new_dummy_field: 100,
            epoch,
            protocol_version,
            system_state_version: SYSTEM_STATE_VERSION_V2,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        }
    }
    
    public(package) fun store_execution_time_estimates(self: &mut HaneulSystemStateInnerV2, estimates: vector<u8>) {
        if (bag::contains(&self.extra_fields, EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_KEY)) {
            let _: vector<u8> = bag::remove(&mut self.extra_fields, EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_KEY);
        };
        bag::add(&mut self.extra_fields, EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_KEY, estimates);
    }

    public(package) fun store_execution_time_estimates_v2(
        self: &mut HaneulSystemStateInnerV2,
        estimate_chunks: vector<vector<u8>>,
    ) {
        let old_key = EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_KEY;
        if (bag::contains(&self.extra_fields, old_key)) {
            let _: vector<u8> = bag::remove(&mut self.extra_fields, old_key);
        };
        let chunk_count_key = EXTRA_FIELD_EXECUTION_TIME_ESTIMATES_CHUNK_COUNT_KEY;
        if (bag::contains(&self.extra_fields, chunk_count_key)) {
            let existing_chunk_count: u64 = bag::remove(&mut self.extra_fields, chunk_count_key);
            let mut chunk_idx = 0;
            while (chunk_idx < existing_chunk_count) {
                let chunk_key = ExecutionTimeObservationChunkKey { chunk_index: chunk_idx };
                if (bag::contains(&self.extra_fields, chunk_key)) {
                    let _: vector<u8> = bag::remove(&mut self.extra_fields, chunk_key);
                };
                chunk_idx = chunk_idx + 1;
            };
        };
        let total_chunks = vector::length(&estimate_chunks);
        if (total_chunks > 0) {
            bag::add(&mut self.extra_fields, chunk_count_key, total_chunks);
            let mut i = 0;
            while (i < total_chunks) {
                let chunk_key = ExecutionTimeObservationChunkKey { chunk_index: i };
                let chunk_data = *vector::borrow(&estimate_chunks, i);
                bag::add(&mut self.extra_fields, chunk_key, chunk_data);
                i = i + 1;
            };
        };
    }
}
