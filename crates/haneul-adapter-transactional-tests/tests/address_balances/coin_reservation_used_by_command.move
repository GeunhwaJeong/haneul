// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// A legacy coin reservation input consumed by a command of the PTB. Only the
// conversion to a `Coin` is injected, ahead of the original command.

//# init --addresses test=0x0 --accounts A B

// Seed A's address balance.
//# programmable --sender A --inputs 100000000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# programmable --sender A --inputs coin_reservation<haneul::balance::Balance<haneul::haneul::HANEUL>>(500000000) @B
//> TransferObjects([Input(0)], Input(1))

//# create-checkpoint

// The reserved amount left A's balance as a coin now owned by B.
//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A
