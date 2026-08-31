// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests gasless balance transfer using a custom stablecoin.

//# init --addresses test=0x0 --accounts A B --enable-gasless

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

//# programmable --sender A --inputs 10000 object(1,1) @A
// Mint 10000 USDC and send to A's address balance
//> 0: haneul::coin::mint<test::usdc::USDC>(Input(1), Input(0));
//> 1: haneul::coin::into_balance<test::usdc::USDC>(Result(0));
//> 2: haneul::balance::send_funds<test::usdc::USDC>(Result(1), Input(2));

//# create-checkpoint

//# view-funds haneul::balance::Balance<test::usdc::USDC> A

//# programmable --sender A --address-balance-gas --gas-price 0 --gas-budget 0 --inputs withdraw<haneul::balance::Balance<test::usdc::USDC>>(1000) @B
// Gasless: A withdraws 1000 USDC and sends to B
//> 0: haneul::balance::redeem_funds<test::usdc::USDC>(Input(0));
//> 1: haneul::balance::send_funds<test::usdc::USDC>(Result(0), Input(1));

//# create-checkpoint

//# view-funds haneul::balance::Balance<test::usdc::USDC> A

//# view-funds haneul::balance::Balance<test::usdc::USDC> B
