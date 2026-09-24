// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// One allowance declared twice in the same transaction with different
// funders. The first declaration resolves and caches the allowance; the
// second must still get its own funder check, so the whole tx is rejected.

//# init --accounts A B C

//# programmable --sender A --inputs 5000 @A
// Fund A's (the funder) address balance.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));

//# programmable --sender C --inputs 1000 @C
// Fund C, so the mismatched declaration gets past the balance check and
// reaches the funder comparison.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));

//# create-checkpoint

//# programmable --sender A --inputs b"dup" @B vector[10000u256] vector[] vector[99999999999999]
// A issues an allowance to B: 10000 lifetime cap.
//> 0: std::option::none<haneul::allowance::RateLimit>();
//> 1: haneul::allowance::new<haneul::balance::Balance<haneul::haneul::HANEUL>>(Input(0), Input(1), Input(2), Input(3), Input(4), Result(0));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@C,object(4,0)) mutshared(4,0) immshared(6)
// Correct funder first, wrong funder second: rejected at signing.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(2), Input(0), Input(3));
//> 1: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(2), Input(1), Input(3));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@C,object(4,0)) allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0) immshared(6)
// Same pair, wrong funder first: also rejected.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(2), Input(0), Input(3));
//> 1: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(2), Input(1), Input(3));

//# view-object 4,0
// current_spend is untouched.
