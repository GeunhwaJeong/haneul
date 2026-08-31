// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Test send_funds and redeem_funds from haneul::balance

//# init --addresses test=0x0 --accounts A B

// Send 1000 from A to B
//# programmable --sender A --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B

// B withdraws 500 and send to A
//# programmable --sender B --inputs withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(500) @A
//> 0: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(0));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));


//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B

// B withdraws 500 and send to self
//# programmable --sender B --inputs withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(500) @B
//> 0: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(0));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));


//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B
