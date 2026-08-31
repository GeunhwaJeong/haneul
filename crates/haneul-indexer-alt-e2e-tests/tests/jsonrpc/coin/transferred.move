// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Transfer an object from A to B and verify that the coin is now included in the result of haneulx_getCoins
// for B and not included in the result for A.

//# init --protocol-version 108 --addresses Test=0x0 --accounts A B --simulator --consistent-range 2

//# programmable --sender A --inputs 12 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs 34 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# view-object 1,0

//# view-object 2,0

//# create-checkpoint

//# run-jsonrpc
{
  "method": "haneulx_getCoins",
  "params": ["@{A}"]
}

//# transfer-object 1,0 --sender A --recipient B

//# create-checkpoint

//# run-jsonrpc
{
  "method": "haneulx_getCoins",
  "params": ["@{A}"]
}

//# run-jsonrpc
{
  "method": "haneulx_getCoins",
  "params": ["@{B}"]
}
