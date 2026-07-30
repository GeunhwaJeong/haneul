// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Tests that --address-balance-gas pays for gas from the address balance,
// leaving owned gas objects untouched.

//# init --addresses test=0x0 --accounts A

// View gas coin before any transactions
//# view-object 0,0

// First send funds to A's address balance so we can pay for gas from it
//# programmable --sender A --inputs 10000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

// Empty transaction using address balance gas
//# programmable --sender A --address-balance-gas

// Use the object, but not as gas
//# programmable --sender A --address-balance-gas --inputs object(0,0)

// View gas coin after -- balance should be unchanged (except for the initial send_funds tx)
//# view-object 0,0

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A
