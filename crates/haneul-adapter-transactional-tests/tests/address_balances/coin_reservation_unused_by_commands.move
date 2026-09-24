// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// A legacy coin reservation input that no command of the PTB uses. The
// conversion to a `Coin` is injected ahead of the original command and the
// send-back of the unused coin is injected after it.

//# init --addresses test=0x0 --accounts A B

// Seed A's address balance.
//# programmable --sender A --inputs 100000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# programmable --sender A --inputs coin_reservation<haneul::balance::Balance<haneul::haneul::HANEUL>>(500000000)
//> haneul::coin::value<haneul::haneul::HANEUL>(Gas)

//# create-checkpoint

// The reservation is withdrawn and sent back, so A's balance is unchanged.
//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A
