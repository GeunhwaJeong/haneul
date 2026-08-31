// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --accounts A B --addresses test=0x0 --simulator

// 1. Show the owner of an object owned by one address
// 2. ...owned by another address
// 3. ...shared
// 4. ...frozen
// 5. ...owned by an object

//# programmable --sender A --inputs 42 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs 43 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: TransferObjects([Result(0)], Input(1))

//# programmable --sender A --inputs 44
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::transfer::public_share_object<haneul::coin::Coin<haneul::haneul::HANEUL>>(Result(0))

//# programmable --sender A --inputs 45
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::transfer::public_freeze_object<haneul::coin::Coin<haneul::haneul::HANEUL>>(Result(0))

//# programmable --sender A --inputs @A
//> 0: haneul::table::new<u64, u64>();
//> 1: TransferObjects([Result(0)], Input(0))

//# programmable --sender A --inputs object(5,0) 46 47
//> 0: haneul::table::add<u64, u64>(Input(0), Input(1), Input(2))

//# create-checkpoint

//# run-jsonrpc
{
  "method": "haneul_tryGetPastObject",
  "params": ["@{obj_1_0}", 2, { "showOwner": true }]
}

//# run-jsonrpc
{
  "method": "haneul_tryGetPastObject",
  "params": ["@{obj_2_0}", 3, { "showOwner": true }]
}

//# run-jsonrpc
{
  "method": "haneul_tryGetPastObject",
  "params": ["@{obj_3_0}", 4, { "showOwner": true }]
}

//# run-jsonrpc
{
  "method": "haneul_tryGetPastObject",
  "params": ["@{obj_4_0}", 5, { "showOwner": true }]
}

//# run-jsonrpc
{
  "method": "haneul_tryGetPastObject",
  "params": ["@{obj_6_0}", 7, { "showOwner": true }]
}
