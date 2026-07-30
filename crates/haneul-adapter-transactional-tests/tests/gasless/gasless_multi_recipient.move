// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Tests gasless multi-recipient transfer using withdrawal_split.

//# init --addresses test=0x0 --accounts A B C --enable-gasless

//# publish --sender A
#[allow(deprecated_usage)]
module test::usdc {
    use haneul::coin;

    public struct USDC has drop {}

    fun init(otw: USDC, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw, 6, b"USDC", b"USD Coin", b"", option::none(), ctx,
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, ctx.sender());
    }
}

//# gasless-allow-token test::usdc::USDC

//# programmable --sender A --inputs 1000 object(1,1) @A
// Mint 1000 USDC and fund A
//> 0: haneul::coin::mint<test::usdc::USDC>(Input(1), Input(0));
//> 1: haneul::coin::into_balance<test::usdc::USDC>(Result(0));
//> 2: haneul::balance::send_funds<test::usdc::USDC>(Result(1), Input(2));

//# create-checkpoint

//# view-funds haneul::balance::Balance<test::usdc::USDC> A

//# programmable --sender A --address-balance-gas --gas-price 0 --gas-budget 0 --inputs withdraw<haneul::balance::Balance<test::usdc::USDC>>(1000) 400u256 @B @C
// Gasless: A splits withdrawal - 400 to B, remainder (600) to C
//> 0: haneul::funds_accumulator::withdrawal_split<haneul::balance::Balance<test::usdc::USDC>>(Input(0), Input(1));
//> 1: haneul::balance::redeem_funds<test::usdc::USDC>(Result(0));
//> 2: haneul::balance::send_funds<test::usdc::USDC>(Result(1), Input(2));
//> 3: haneul::balance::redeem_funds<test::usdc::USDC>(Input(0));
//> 4: haneul::balance::send_funds<test::usdc::USDC>(Result(3), Input(3));

//# create-checkpoint

//# view-funds haneul::balance::Balance<test::usdc::USDC> A

//# view-funds haneul::balance::Balance<test::usdc::USDC> B

//# view-funds haneul::balance::Balance<test::usdc::USDC> C
