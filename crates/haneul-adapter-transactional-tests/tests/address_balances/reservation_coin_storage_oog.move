// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Storage-OOG reset + re-smash for `[AddressBalance, Coin]` gas payment.
// The secondary coin is deleted during the first smash, so reset has to
// un-delete it before the re-smash can re-read its value and re-delete it.
// Also clears the Merge deposit-back accumulator event before it's
// re-emitted.

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

// Create a coin to be smashed into the withdrawal target.
//# programmable --sender A --inputs 1000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

// [AddressBalance, Coin] payment with tight budget; workload OOGs storage.
// On reset, the secondary coin (object 3,0) must be un-deleted so the
// re-smash can read its value again.
//# programmable --sender A --gas-budget 5000000 --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(5000000) --gas-payment object(3,0) --inputs 100
//> test::oog::make(Input(0))

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

// The secondary coin should be gone after the (re-smashed) tx succeeds in
// re-deleting it.
//# view-object 3,0
