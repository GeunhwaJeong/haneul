// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// AllowanceWithdrawal<T> and Withdrawal<T> are distinct input types: each
// redeem path accepts only its own, so cross-use fails at typing.

//# init --accounts A B

//# programmable --sender A --inputs 5000 @A
// Fund A's (the funder) address balance.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));

//# programmable --sender B --inputs 1000 @B
// Fund B's (the spender) address balance.
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::send_funds<haneul::haneul::HANEUL>(Result(0), Input(1));

//# create-checkpoint

//# programmable --sender A --inputs b"txn_test" @B vector[1000u256] vector[] vector[99999999999999]
// A issues an allowance to B: 1000 lifetime cap, no rate limit.
//> 0: std::option::none<haneul::allowance::RateLimit>();
//> 1: haneul::allowance::new<haneul::balance::Balance<haneul::haneul::HANEUL>>(Input(0), Input(1), Input(2), Input(3), Input(4), Result(0));

//# view-object 4,0

//# view-object 4,1

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(400,@A,object(4,0)) mutshared(4,0) immshared(6) @B
// Happy path: B spends 400 through the allowance.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(1), Input(0), Input(2));
//> 1: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(0), Input(3));

//# view-object 4,0
// current_spend is now 400.

//# programmable --sender B --inputs withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100) mutshared(4,0) immshared(6)
// A sender Withdrawal where an AllowanceWithdrawal is expected: typing rejects.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(1), Input(0), Input(2));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0)
// An AllowanceWithdrawal where a Withdrawal is expected: typing rejects.
//> 0: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(0));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0)
// Same through coin::redeem_funds.
//> 0: haneul::coin::redeem_funds<haneul::haneul::HANEUL>(Input(0));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0) immshared(6)
// The AllowanceWithdrawal where the &mut Allowance is expected: typing rejects.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(0), Input(0), Input(2));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0) immshared(6)
// The Allowance object where the AllowanceWithdrawal is expected: typing rejects.
//> 0: haneul::allowance::balance_spend<haneul::haneul::HANEUL>(Input(1), Input(1), Input(2));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0)
// The Allowance object into the plain redeem API: typing rejects.
//> 0: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(1));

//# programmable --sender B --inputs allowance_withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(100,@A,object(4,0)) mutshared(4,0) 1 @B
// Declaring an allowance withdrawal and never using it consumes nothing.
//> 0: SplitCoins(Gas, [Input(2)]);
//> 1: TransferObjects([Result(0)], Input(3));

//# view-object 4,0
// current_spend is unchanged.
