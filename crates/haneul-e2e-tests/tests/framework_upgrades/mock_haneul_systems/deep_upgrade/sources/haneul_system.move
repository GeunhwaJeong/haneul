// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module haneul_system::haneul_system {
    use std::vector;

    use haneul::balance::Balance;
    use haneul::object::UID;
    use haneul::haneul::HANEUL;
    use haneul::transfer;
    use haneul::tx_context::{Self, TxContext};
    use haneul::dynamic_field;

    use haneul_system::validator::Validator;
    use haneul_system::haneul_system_state_inner::{Self, HaneulSystemStateInnerV2, HaneulSystemStateInner};

    public struct HaneulSystemState has key {
        id: UID,
        version: u64,
    }

    public(package) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<HANEUL>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = haneul_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = haneul_system_state_inner::genesis_system_state_version();
        let mut self = HaneulSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<HANEUL>,
        computation_reward: Balance<HANEUL>,
        wrapper: &mut HaneulSystemState,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        _reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<HANEUL> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x0, 0);
        let storage_rebate = haneul_system_state_inner::advance_epoch(
            self,
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            epoch_start_timestamp_ms,
        );

        storage_rebate
    }

    public fun active_validator_addresses(wrapper: &mut HaneulSystemState): vector<address> {
        vector[]
    }

    fun load_system_state_mut(self: &mut HaneulSystemState): &mut HaneulSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut HaneulSystemState): &mut HaneulSystemStateInnerV2 {
        let mut version = self.version;
        if (version == haneul_system_state_inner::genesis_system_state_version()) {
            let inner: HaneulSystemStateInner = dynamic_field::remove(&mut self.id, version);
            let new_inner = haneul_system_state_inner::v1_to_v2(inner);
            version = haneul_system_state_inner::system_state_version(&new_inner);
            dynamic_field::add(&mut self.id, version, new_inner);
            self.version = version;
        };

        let inner: &mut HaneulSystemStateInnerV2 = dynamic_field::borrow_mut(&mut self.id, version);
        assert!(haneul_system_state_inner::system_state_version(inner) == version, 0);
        inner
    }

    fun store_execution_time_estimates(wrapper: &mut HaneulSystemState, estimates_bytes: vector<u8>) {
        let self = load_system_state_mut(wrapper);
        haneul_system_state_inner::store_execution_time_estimates(self, estimates_bytes)
    }

    fun store_execution_time_estimates_v2(wrapper: &mut HaneulSystemState, estimate_chunks: vector<vector<u8>>) {
        let self = load_system_state_mut(wrapper);
        haneul_system_state_inner::store_execution_time_estimates_v2(self, estimate_chunks)
    }

    fun write_accumulator_storage_cost(
        _wrapper: &mut HaneulSystemState,
        _storage_cost: u64,
        _ctx: &TxContext,
    ) {
    }
}
