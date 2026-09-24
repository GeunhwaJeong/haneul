// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --addresses test=0x0 --accounts A --enable-feature-flags check_object_funds_withdraw_in_execution

//# publish --sender A
module test::obj_vault;

use haneul::balance;
use haneul::coin::Coin;
use haneul::haneul::HANEUL;

public struct Vault has key {
    id: UID,
}

public fun new(ctx: &mut TxContext) {
    transfer::transfer(Vault { id: object::new(ctx) }, ctx.sender());
}

public fun fund(vault: &Vault, coin: Coin<HANEUL>) {
    balance::send_funds<HANEUL>(coin.into_balance(), vault.id.to_address());
}

public fun withdraw_to(vault: &mut Vault, amount: u64, recipient: address) {
    let w = balance::withdraw_funds_from_object<HANEUL>(&mut vault.id, amount);
    let bal = balance::redeem_funds<HANEUL>(w);
    balance::send_funds<HANEUL>(bal, recipient);
}

//# run test::obj_vault::new --sender A

//# programmable --sender A --dry-run --inputs object(2,0) 0 @A
// Withdrawing zero GEUNHWA should succeed before the vault has an object balance.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --dry-run --inputs object(2,0) 1 @A
// Withdrawing one GEUNHWA should fail before the vault has an object balance.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --inputs 1000 object(2,0)
// Fund the object balance vault with 1000 GEUNHWA.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::obj_vault::fund(Input(1), Result(0));

//# create-checkpoint
// Make sure the valut funding settles.

//# programmable --sender A --dry-run --inputs object(2,0) 500 @A
// Withdrawing 500 GEUNHWA from the funded vault should succeed.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --dry-run --inputs object(2,0) 5000 @A
// Withdrawing more than the vault's balance should fail.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --dry-run --inputs object(2,0) 600 @A
// Two withdrawals whose sum exceeds the vault's balance should fail on the second withdrawal.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));
//> 1: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --dry-run --inputs 500 object(2,0) 1400 @A
// Funds deposited in this transaction should be available to withdraw, allowing a 1400 GEUNHWA withdrawal.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::obj_vault::fund(Input(1), Result(0));
//> 2: test::obj_vault::withdraw_to(Input(1), Input(2), Input(3));

//# programmable --sender A --dry-run --inputs 500 object(2,0) 1600 @A
// Withdrawing more than the vault's balance plus funds deposited in this transaction should fail.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::obj_vault::fund(Input(1), Result(0));
//> 2: test::obj_vault::withdraw_to(Input(1), Input(2), Input(3));

//# programmable --sender A --dry-run --inputs object(2,0) 0 @A
// Withdrawing zero GEUNHWA should succeed.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --dry-run --inputs 700 object(2,0) 600 @A
// The vault can be funded and partially withdrawn in the same transaction.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::obj_vault::fund(Input(1), Result(0));
//> 2: test::obj_vault::withdraw_to(Input(1), Input(2), Input(3));

//# programmable --sender A --inputs object(2,0) 600 @A
// The first committed withdrawal of 600 GEUNHWA should succeed.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --inputs object(2,0) 600 @A
// A second 600 GEUNHWA withdrawal should fail because only 400 GEUNHWA remains.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# create-checkpoint
// After this settlement, the vault has 400 GEUNHWA remaining.

//# programmable --sender A --inputs object(2,0) 400 @A
// Withdrawing the remaining 400 GEUNHWA after a checkpoint should succeed.
//> 0: test::obj_vault::withdraw_to(Input(0), Input(1), Input(2));

//# programmable --sender A --inputs 500 object(2,0) 400 @A
// After the vault is emptied, funding it with 500 GEUNHWA and withdrawing 400 GEUNHWA should leave 100 GEUNHWA.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::obj_vault::fund(Input(1), Result(0));
//> 2: test::obj_vault::withdraw_to(Input(1), Input(2), Input(3));
