// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// A self-allowance: one address as funder, spender, and sender. Spends work,
// and an allowance withdrawal and a plain sender withdrawal coalesce at the
// same (address, type) reservation key.

//# init --accounts A B

//# programmable --sender A --inputs 5000 @A
// Fund A's (the funder) address balance.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));

//# create-checkpoint

//# programmable --sender A --inputs b"self" @A vector[1000u256] vector[] vector[99999999999999]
// A issues an allowance to themselves: 1000 lifetime cap.
//> 0: std::option::none<haneul::allowance::RateLimit>();
//> 1: haneul::allowance::new<haneul::balance::Balance<haneul::haneul::HANEUL>>(Input(0), Input(1), Input(2), Input(3), Input(4), Result(0));

//# programmable --sender A --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(400,@A,object(3,0)) mutshared(3,0) immshared(6) @B
// A spends 400 through their own allowance.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(1), Input(0), Input(2));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(3));

//# programmable --sender A --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(200,@A,object(3,0)) withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) mutshared(3,0) immshared(6) @B
// An allowance withdrawal and a plain withdrawal, both against A's balance:
// the reservations coalesce at one key and settle as one Split.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(2), Input(0), Input(3));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(4));
//> 2: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(1));
//> 3: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(2), Input(4));

//# view-object 3,0
// current_spend is 600: the allowance-sourced 400 + 200 only.

//# create-checkpoint

//# view-object 1,0
// A's settled balance: 5000 - 400 - 300 = 4300.

//# view-object 4,0
// B's settled balance: 400 + 300 = 700.

//# programmable --sender A --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(3,0)) mutshared(3,0) object(3,1) immshared(6) @B
// Spend and revoke in one PTB: the withdrawal consumes first, then the
// revoke deletes the allowance and its cap.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(1), Input(0), Input(3));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(4));
//> 2: haneul::allowance::revoke<haneul::balance::Balance<haneul::haneul::HANEUL>>(Input(2), Input(1));

//# programmable --sender A --inputs b"self2" @A vector[1000u256] vector[] vector[99999999999999]
// A second self-allowance, for the reversed order below.
//> 0: std::option::none<haneul::allowance::RateLimit>();
//> 1: haneul::allowance::new<haneul::balance::Balance<haneul::haneul::HANEUL>>(Input(0), Input(1), Input(2), Input(3), Input(4), Result(0));

//# programmable --sender A --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(11,0)) mutshared(11,0) object(11,1) immshared(6) @B
// Revoke first: the spend cannot use the consumed allowance, so the whole
// transaction rolls back, revocation included.
//> 0: haneul::allowance::revoke<haneul::balance::Balance<haneul::haneul::HANEUL>>(Input(2), Input(1));
//> 1: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(1), Input(0), Input(3));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(4));

//# view-object 11,0
// The second allowance survives the failed revoke; current_spend is 0.

//# create-checkpoint

//# view-object 1,0
// A's settled balance: 4300 - 100 = 4200.

//# view-object 4,0
// B's settled balance: 700 + 100 = 800.
