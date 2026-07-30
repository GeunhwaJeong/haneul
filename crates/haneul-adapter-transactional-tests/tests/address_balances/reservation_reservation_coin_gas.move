// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// `[AddressBalance, AddressBalance, Coin]` (same address). The two
// withdrawals share a payment location and are summed into a single entry
// before smashing runs. Smashing then sees `[AddressBalance(merged), Coin]`.
// The snapshot must show only one withdrawal-side accumulator event for the
// merged reservation (not two), confirming the dedup happens upfront.

//# init --addresses test=0x0 --accounts A B

//# view-object 0,0

// Seed A's address balance.
//# programmable --sender A --inputs 100000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

// Create a HANEUL coin to be smashed into the (deduped) withdrawal target.
//# programmable --sender A --inputs 1000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

// Pay with `[AddressBalance, AddressBalance, Coin]`. The two 500M reservations
// dedup into a single 1B entry. Coin (object 3,0) gets deleted.
// total_smashed = 1_000_000_000 + 1_000_000_000 = 2_000_000_000;
// deposit-back = 1_000_000_000.
//# programmable --sender A --gas-budget 1000000000 --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(500000000) --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(500000000) --gas-payment object(3,0) --inputs 100000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# create-checkpoint

// Final balance reflects: start + 1B deposit-back - net gas charge.
//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A
