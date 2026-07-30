// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Benchmark the send_funds command with a HANEUL withdrawal from A's address balance
// sent to B, with gas also paid from A's address balance.

//# init --addresses test=0x0 --accounts A B C D E

// Seed A's address balance so it can both fund the withdrawal and pay for gas.
//# programmable --sender A --inputs 20000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

// Benchmark: withdraw 100 from A's address balance and send_funds to B,
// with gas paid from the address balance.
//# bench ptb --sender A --address-balance-gas --inputs withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) @B
//> 0: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(0));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));

// Benchmark: withdraw 400 from A's address balance and send it to B, C, D, and E
// with gas paid from the address balance.
//# bench ptb --sender A --address-balance-gas --inputs withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) @B @C @D @E
//> 0: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(0));
//> 1: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(1));
//> 2: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(2));
//> 3: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(3));
//> 4: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(4));
//> 5: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(5));
//> 6: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(2), Input(6));
//> 7: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(3), Input(7));

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B
