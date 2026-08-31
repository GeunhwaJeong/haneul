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
    use haneul_system::haneul_system_state_inner::HaneulSystemStateInner;
    use haneul_system::haneul_system_state_inner;

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
        _new_epoch: u64,
        _next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64,
        _reward_slashing_rate: u64,
        _epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) : Balance<HANEUL> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x1, 0); // aborts here
        haneul_system_state_inner::advance_epoch(
            self,
            storage_reward,
            computation_reward,
            storage_rebate,
        )
    }

    public fun active_validator_addresses(wrapper: &mut HaneulSystemState): vector<address> {
        vector[]
    }

    fun load_system_state_mut(self: &mut HaneulSystemState): &mut HaneulSystemStateInner {
        let version = self.version;
        dynamic_field::borrow_mut(&mut self.id, version)
    }

    fun write_accumulator_storage_cost(
        _wrapper: &mut HaneulSystemState,
        _storage_cost: u64,
        _ctx: &TxContext,
    ) {
    }
}
