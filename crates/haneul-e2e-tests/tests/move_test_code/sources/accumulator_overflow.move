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

/// Object-sourced withdrawals are checked for backing only at settlement (which runs after effects
/// construction), so this withdraws `u64::MAX` and deposits it to the sender, yielding a single
/// Move-native Merge of `u64::MAX` to `(sender, Balance<HANEUL>)`.
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

/// Deposit `u64::MAX` of `Balance<T>` to the sender twice via object-sourced withdrawals. Both
/// deposits are Move-native Merges to `(sender, Balance<T>)`, so the second pushes the object-runtime
/// per-key merge total past `u64::MAX` and is rejected with an arithmetic error. Unlike
/// `Balance<HANEUL>`, an arbitrary `Balance<T>` has no uncapped gas-smash deposit path, so this per-key
/// cap is the binding guard (and gas/conservation checking does not apply to non-HANEUL types).
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
