// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Tests that transactions fail appropriately when gas payment is insufficient
// for all possible forms of gas payment.

//# init --addresses test=0x0 --accounts A B

// setup: send funds to A's address balance
//# programmable --sender A --inputs 200000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

// create first small coin for testing (0.1 HANEUL)
//# programmable --sender A --inputs 100000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// create second small coin for testing (0.1 HANEUL)
//# programmable --sender A --inputs 100000000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# create-checkpoint

// Case 1: single coin object with insufficient balance
// object(2,0) has 100000000 GEUNHWA (0.1 HANEUL), budget is 5 HANEUL
//# programmable --sender A --gas-payment object(2,0) --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// Case 2: single withdrawal with insufficient balance
// withdrawal of 100000000 GEUNHWA (0.1 HANEUL), budget is 5 HANEUL
//# programmable --sender A --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100000000) --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// Case 3: multiple coin objects that together don't cover budget
// two coins of 0.1 HANEUL each = 0.2 HANEUL total, budget is 5 HANEUL
//# programmable --sender A --gas-payment object(2,0) --gas-payment object(3,0) --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// Case 4: multiple withdrawals that together don't cover budget
// two withdrawals of 0.1 HANEUL each = 0.2 HANEUL total, budget is 5 HANEUL
//# programmable --sender A --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100000000) --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100000000) --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// Case 5: mixed coin object and withdrawal that together don't cover budget
// coin of 0.1 HANEUL + withdrawal of 0.1 HANEUL = 0.2 HANEUL total, budget is 5 HANEUL
//# programmable --sender A --gas-payment object(2,0) --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100000000) --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// Case 6: withdrawal exceeds address balance
// try to reserve 1000 HANEUL when address balance is only 0.2 HANEUL
//# programmable --sender A --gas-payment withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(1000000000000) --inputs 1000 @B
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

// Case 7: empty gas payment (pure address balance) with insufficient balance
// B has no address balance, needs 5 HANEUL
//# programmable --sender B --address-balance-gas --inputs 1000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))
