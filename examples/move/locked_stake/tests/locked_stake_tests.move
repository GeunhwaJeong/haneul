// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(deprecated_usage)] // TODO: update tests to not use deprecated governance
module locked_stake::locked_stake_tests;

use locked_stake::epoch_time_lock;
use locked_stake::locked_stake as ls;
use std::unit_test::{assert_eq, destroy};
use haneul::balance;
use haneul::coin;
use haneul::test_scenario;
use haneul::vec_map;
use haneul_system::governance_test_utils::{advance_epoch, set_up_haneul_system_state};
use haneul_system::haneul_system::{Self, HaneulSystemState};

const GEUNHWA_PER_HANEUL: u64 = 1_000_000_000;

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochAlreadyPassed)]
fun test_incorrect_creation() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_haneul_system_state(vector[@0x1, @0x2, @0x3]);

    // Advance epoch twice so we are now at epoch 2.
    advance_epoch(scenario);
    advance_epoch(scenario);
    let ctx = test_scenario::ctx(scenario);
    assert_eq!(tx_context::epoch(ctx), 2);

    // Create a locked stake with epoch 1. Should fail here.
    let ls = ls::new(1, ctx);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_deposit_stake_unstake() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_haneul_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(10, test_scenario::ctx(scenario));

    // Deposit 100 HANEUL.
    ls::deposit_haneul(&mut ls, balance::create_for_testing(100 * GEUNHWA_PER_HANEUL));

    assert_eq!(ls::haneul_balance(&ls), 100 * GEUNHWA_PER_HANEUL);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(scenario);

    // Stake 10 of the 100 HANEUL.
    ls::stake(&mut ls, &mut system_state, 10 * GEUNHWA_PER_HANEUL, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    assert_eq!(ls::haneul_balance(&ls), 90 * GEUNHWA_PER_HANEUL);
    assert_eq!(vec_map::length(ls::staked_haneul(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
    let ctx = test_scenario::ctx(scenario);

    // Create a StakedHaneul object and add it to the LockedStake object.
    let staked_haneul = haneul_system::request_add_stake_non_entry(
        &mut system_state,
        coin::mint_for_testing(20 * GEUNHWA_PER_HANEUL, ctx),
        @0x2,
        ctx,
    );
    test_scenario::return_shared(system_state);

    ls::deposit_staked_haneul(&mut ls, staked_haneul);
    assert_eq!(ls::haneul_balance(&ls), 90 * GEUNHWA_PER_HANEUL);
    assert_eq!(vec_map::length(ls::staked_haneul(&ls)), 2);
    advance_epoch(scenario);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_haneul_id, _) = vec_map::get_entry_by_idx(ls::staked_haneul(&ls), 0);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(scenario);

    // Unstake both stake objects
    ls::unstake(&mut ls, &mut system_state, *staked_haneul_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq!(ls::haneul_balance(&ls), 100 * GEUNHWA_PER_HANEUL);
    assert_eq!(vec_map::length(ls::staked_haneul(&ls)), 1);

    test_scenario::next_tx(scenario, @0x1);
    let (staked_haneul_id, _) = vec_map::get_entry_by_idx(ls::staked_haneul(&ls), 0);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
    ls::unstake(&mut ls, &mut system_state, *staked_haneul_id, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);
    assert_eq!(ls::haneul_balance(&ls), 120 * GEUNHWA_PER_HANEUL);
    assert_eq!(vec_map::length(ls::staked_haneul(&ls)), 0);

    destroy(ls);
    test_scenario::end(scenario_val);
}

#[test]
fun test_unlock_correct_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_haneul_system_state(vector[@0x1, @0x2, @0x3]);

    let mut ls = ls::new(2, test_scenario::ctx(scenario));

    ls::deposit_haneul(&mut ls, balance::create_for_testing(100 * GEUNHWA_PER_HANEUL));

    assert_eq!(ls::haneul_balance(&ls), 100 * GEUNHWA_PER_HANEUL);

    test_scenario::next_tx(scenario, @0x1);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(scenario);
    ls::stake(&mut ls, &mut system_state, 10 * GEUNHWA_PER_HANEUL, @0x1, test_scenario::ctx(scenario));
    test_scenario::return_shared(system_state);

    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);
    advance_epoch(scenario);

    let (staked_haneul, haneul_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    assert_eq!(balance::value(&haneul_balance), 90 * GEUNHWA_PER_HANEUL);
    assert_eq!(vec_map::length(&staked_haneul), 1);

    destroy(staked_haneul);
    destroy(haneul_balance);
    test_scenario::end(scenario_val);
}

#[test]
#[expected_failure(abort_code = epoch_time_lock::EEpochNotYetEnded)]
fun test_unlock_incorrect_epoch() {
    let mut scenario_val = test_scenario::begin(@0x0);
    let scenario = &mut scenario_val;

    set_up_haneul_system_state(vector[@0x1, @0x2, @0x3]);

    let ls = ls::new(2, test_scenario::ctx(scenario));
    let (staked_haneul, haneul_balance) = ls::unlock(ls, test_scenario::ctx(scenario));
    destroy(staked_haneul);
    destroy(haneul_balance);
    test_scenario::end(scenario_val);
}
