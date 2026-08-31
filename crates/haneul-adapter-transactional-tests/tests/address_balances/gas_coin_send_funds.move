// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests passing Gas by value to haneul::coin::send_funds in various scenarios.

//# init --addresses test=0x0 --accounts A B C D E

//# programmable --sender A --inputs @B --gas-budget 10000000
// Send gas coin to another address via send_funds
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0))

//# view-object 0,0

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B


//# programmable --sender B --inputs @B --gas-budget 10000000
// Send gas coin to self via send_funds, the gas coin should be deleted
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0))

//# create-checkpoint

//# view-object 0,1

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B


//# programmable --sender B --inputs @B --address-balance-gas --gas-budget 10000000
// Send ephemeral gas coin to self via send_funds
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0))

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B


//# programmable --sender B --inputs @C --address-balance-gas --gas-budget 10000000
// Send ephemeral gas coin to another address via send_funds
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0))

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> C


//# programmable --sender C --inputs @D 10 --gas-budget 10000000
// Send the gas coin via send_funds, but split from it first
//> 0: SplitCoins(Gas, [Input(1)]);
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0));
//> TransferObjects([Result(0)], Input(0))

//# view-object 0,2

//# view-object 17,0

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> C

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> D

//# programmable --sender D --inputs @E 0 --address-balance-gas --gas-budget 10000000
// Send the ephemeral gas coin via send_funds, but split from it first
//> 0: SplitCoins(Gas, [Input(1)]);
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0));
//> TransferObjects([Result(0)], Input(0))

//# view-object 0,2

//# view-object 23,0

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> D

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> E
