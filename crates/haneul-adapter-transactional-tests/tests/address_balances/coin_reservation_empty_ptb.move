// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// A legacy coin reservation input in a PTB with no commands of its own. The
// conversion to a `Coin` and the send-back of the unused coin are both injected
// during typing, so every executed command is annotated with an original command
// index that does not exist.

//# init --addresses test=0x0 --accounts A B

// Seed A's address balance.
//# programmable --sender A --inputs 100000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# programmable --sender A --inputs coin_reservation<haneul::balance::Balance<haneul::haneul::HANEUL>>(500000000)

//# create-checkpoint

// The reservation is withdrawn and immediately sent back, so A's balance is unchanged.
//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A
