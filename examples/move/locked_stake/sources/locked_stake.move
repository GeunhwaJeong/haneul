// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module locked_stake::locked_stake;

use locked_stake::epoch_time_lock::{Self, EpochTimeLock};
use haneul::balance::{Self, Balance};
use haneul::coin;
use haneul::haneul::HANEUL;
use haneul::vec_map::{Self, VecMap};
use haneul_system::staking_pool::StakedHaneul;
use haneul_system::haneul_system::{Self, HaneulSystemState};

const EInsufficientBalance: u64 = 0;
const EStakeObjectNonExistent: u64 = 1;

/// An object that locks HANEUL tokens and stake objects until a given epoch, and allows
/// staking and unstaking operations when locked.
public struct LockedStake has key {
    id: UID,
    staked_haneul: VecMap<ID, StakedHaneul>,
    haneul: Balance<HANEUL>,
    locked_until_epoch: EpochTimeLock,
}

// ============================= basic operations =============================

/// Create a new LockedStake object with empty staked_haneul and haneul balance given a lock time.
/// Aborts if the given epoch has already passed.
public fun new(locked_until_epoch: u64, ctx: &mut TxContext): LockedStake {
    LockedStake {
        id: object::new(ctx),
        staked_haneul: vec_map::empty(),
        haneul: balance::zero(),
        locked_until_epoch: epoch_time_lock::new(locked_until_epoch, ctx),
    }
}

/// Unlocks and returns all the assets stored inside this LockedStake object.
/// Aborts if the unlock epoch is in the future.
public fun unlock(ls: LockedStake, ctx: &TxContext): (VecMap<ID, StakedHaneul>, Balance<HANEUL>) {
    let LockedStake { id, staked_haneul, haneul, locked_until_epoch } = ls;
    epoch_time_lock::destroy(locked_until_epoch, ctx);
    object::delete(id);
    (staked_haneul, haneul)
}

/// Deposit a new stake object to the LockedStake object.
public fun deposit_staked_haneul(ls: &mut LockedStake, staked_haneul: StakedHaneul) {
    let id = object::id(&staked_haneul);
    // This insertion can't abort since each object has a unique id.
    vec_map::insert(&mut ls.staked_haneul, id, staked_haneul);
}

/// Deposit haneul balance to the LockedStake object.
public fun deposit_haneul(ls: &mut LockedStake, haneul: Balance<HANEUL>) {
    balance::join(&mut ls.haneul, haneul);
}

/// Take `amount` of HANEUL from the haneul balance, stakes it, and puts the stake object
/// back into the staked haneul vec map.
public fun stake(
    ls: &mut LockedStake,
    haneul_system: &mut HaneulSystemState,
    amount: u64,
    validator_address: address,
    ctx: &mut TxContext,
) {
    assert!(balance::value(&ls.haneul) >= amount, EInsufficientBalance);
    let stake = haneul_system::request_add_stake_non_entry(
        haneul_system,
        coin::from_balance(balance::split(&mut ls.haneul, amount), ctx),
        validator_address,
        ctx,
    );
    deposit_staked_haneul(ls, stake);
}

/// Unstake the stake object with `staked_haneul_id` and puts the resulting principal
/// and rewards back into the locked haneul balance.
/// Returns the amount of HANEUL unstaked, including both principal and rewards.
/// Aborts if no stake exists with the given id.
public fun unstake(
    ls: &mut LockedStake,
    haneul_system: &mut HaneulSystemState,
    staked_haneul_id: ID,
    ctx: &mut TxContext,
): u64 {
    assert!(vec_map::contains(&ls.staked_haneul, &staked_haneul_id), EStakeObjectNonExistent);
    let (_, stake) = vec_map::remove(&mut ls.staked_haneul, &staked_haneul_id);
    let haneul_balance = haneul_system::request_withdraw_stake_non_entry(haneul_system, stake, ctx);
    let amount = balance::value(&haneul_balance);
    deposit_haneul(ls, haneul_balance);
    amount
}

// ============================= getters =============================

public fun staked_haneul(ls: &LockedStake): &VecMap<ID, StakedHaneul> {
    &ls.staked_haneul
}

public fun haneul_balance(ls: &LockedStake): u64 {
    balance::value(&ls.haneul)
}

public fun locked_until_epoch(ls: &LockedStake): u64 {
    epoch_time_lock::epoch(&ls.locked_until_epoch)
}

// TODO: possibly add some scenarios like switching stake, creating a new LockedStake and transferring
// it to the sender, etc. But these can also be done as PTBs.
