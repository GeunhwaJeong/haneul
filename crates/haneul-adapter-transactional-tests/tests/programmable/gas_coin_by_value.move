// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// tests valid gas coin usage by value

//# init --addresses test=0x0 --accounts A B

//# programmable --sender A --inputs @B
//> TransferObjects([Gas], Input(0))

//# view-object 0,0

//# programmable --sender B --inputs @A --gas-payment object(0,0)
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0))

//# view-object 0,0

//# create-checkpoint

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> A

//# view-funds haneul::balance::Balance<haneul::haneul::HANEUL> B
