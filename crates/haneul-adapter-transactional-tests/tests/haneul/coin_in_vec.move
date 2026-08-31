// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --addresses test=0x0 --accounts A

//# publish --sender A

module test::coin_in_vec {
    use haneul::coin::Coin;
    use haneul::haneul::HANEUL;

    public struct Wrapper has key {
        id: UID,
        coins: vector<Coin<HANEUL>>,
    }

    fun init(ctx: &mut TxContext) {
        transfer::transfer(Wrapper { id: object::new(ctx), coins: vector[] }, tx_context::sender(ctx));
    }

    public fun deposit(wrapper: &mut Wrapper, c: Coin<HANEUL>) {
        vector::push_back(&mut wrapper.coins, c)
    }

    public fun withdraw(wrapper: &mut Wrapper, ctx: &mut TxContext) {
        transfer::public_transfer(vector::pop_back(&mut wrapper.coins), tx_context::sender(ctx))
    }
}

//# programmable --sender A --inputs 10 @A
//> SplitCoins(Gas, [Input(0)]);
//> TransferObjects([Result(0)], Input(1))

//# run test::coin_in_vec::deposit --args object(1,1) object(2,0) --sender A

//# run test::coin_in_vec::withdraw --args object(1,1) --sender A
