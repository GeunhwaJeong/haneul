// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

/// Test helpers that produce Move-native accumulator Merge/Split events of large amounts (up to
/// `u64::MAX`) on a balance key, used to exercise the per-key accumulator representability guards.
module move_test_code::accumulator_overflow;

use haneul::balance;
use haneul::coin::{Self, Coin};
use haneul::object::{Self, UID};
use haneul::haneul::HANEUL;
use haneul::tx_context::{Self, TxContext};

const U64_MAX: u64 = 18446744073709551615;

/// Regression helper for the old post-execution object-funds path: this attempts to withdraw
/// `u64::MAX` from a fresh object and deposit it to the sender. With in-execution object-funds
/// checking enabled, this aborts before emitting the `u64::MAX` accumulator writes.
public entry fun merge_u64_max(ctx: &mut TxContext) {
    let sender = tx_context::sender(ctx);
    let mut id = object::new(ctx);

    let w = balance::withdraw_funds_from_object<HANEUL>(&mut id, U64_MAX);
    let bal = balance::redeem_funds<HANEUL>(w);
    balance::send_funds<HANEUL>(bal, sender);

    object::delete(id);
}

/// Withdraw `amount` of HANEUL from a fresh object and return it as a `Coin<HANEUL>`. The per-object
/// withdrawal emits a `Split` of `amount` on that object's accumulator key, which the supply guard
/// bounds to `<= TOTAL_SUPPLY_GEUNHWA`. The returned `Coin` can be merged into `Argument::GasCoin` via
/// a PTB `MergeCoins` command — not an accumulator event — so several such withdrawals can drive the
/// gas coin's raw `u64` value up to `u64::MAX`, beyond what the supply guard permits for any single
/// balance.
public fun withdraw_haneul_as_coin(amount: u64, ctx: &mut TxContext): Coin<HANEUL> {
    let mut id = object::new(ctx);
    let w = balance::withdraw_funds_from_object<HANEUL>(&mut id, amount);
    let bal = balance::redeem_funds<HANEUL>(w);
    object::delete(id);
    coin::from_balance<HANEUL>(bal, ctx)
}

/// Regression helper for a custom-coin accumulator overflow shape. With in-execution object-funds
/// checking enabled, the first unbacked withdrawal aborts before either `u64::MAX` deposit is
/// emitted.
public entry fun double_merge_u64_max<T>(ctx: &mut TxContext) {
    let sender = tx_context::sender(ctx);

    let mut id1 = object::new(ctx);
    let w1 = balance::withdraw_funds_from_object<T>(&mut id1, U64_MAX);
    balance::send_funds<T>(balance::redeem_funds<T>(w1), sender);
    object::delete(id1);

    let mut id2 = object::new(ctx);
    let w2 = balance::withdraw_funds_from_object<T>(&mut id2, U64_MAX);
    balance::send_funds<T>(balance::redeem_funds<T>(w2), sender);
    object::delete(id2);
}
