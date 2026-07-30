// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Storage-OOG reset + re-smash for `[Coin, AddressBalance]` gas payment.
// Smash target is the Coin: reset must un-mutate the gas coin's smashed
// value and clear the secondary's Split withdraw accumulator event; re-smash
// must re-mutate and re-emit consistently.

//# init --addresses test=0x0 --accounts A B

//# publish
module test::oog;
public struct W has key, store { id: UID }
public fun make(n: u64, ctx: &mut TxContext) {
    let mut i = 0;
    while (i < n) {
        haneul::transfer::public_transfer(W { id: object::new(ctx) }, ctx.sender());
        i = i + 1;
    }
}

// Seed A's address balance.
//# programmable --sender A --inputs 100000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

// Mixed [Coin, AddressBalance] payment with tight budget; workload OOGs storage.
//# programmable --sender A --gas-budget 5000000 --gas-payment object(0,0) --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(5000000) --inputs 100
//> test::oog::make(Input(0))

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-object 0,0
