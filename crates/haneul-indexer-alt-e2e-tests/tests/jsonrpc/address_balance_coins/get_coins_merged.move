// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Create both real coins and an address balance for the same owner, verify they are merged in
// descending balance order in haneulx_getCoins.

//# init --protocol-version 119 --addresses Test=0x0 --accounts A B --simulator

//# programmable --sender A --inputs 500 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs 100 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs 300 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

//# run-jsonrpc
{
  "method": "haneulx_getCoins",
  "params": ["@{B}"]
}
