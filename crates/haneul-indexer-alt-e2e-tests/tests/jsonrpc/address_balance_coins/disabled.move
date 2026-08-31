// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Fund an address balance for B and verify that haneulx_getCoins DOES NOT return
// the address balance coin, because the protocol does not support that feature
// yet.

//# init --protocol-version 118 --addresses Test=0x0 --accounts A B --simulator

// Send 1_000_000_000 from A to B's address balance
//# programmable --sender A --inputs 1000000000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

// B should see the address balance coin in getCoins
//# run-jsonrpc
{
  "method": "haneulx_getCoins",
  "params": ["@{B}"]
}
