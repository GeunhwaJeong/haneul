// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests repacking an object that holds HANEUL into a different type, splitting the HANEUL out as a coin.

//# init --addresses ex=0x0 --accounts A --flavor core

//# publish
module ex::m {
    use haneul::balance::Balance;
    use haneul::coin::{Self, Coin};
    use haneul::object::{Self, UID};
    use haneul::haneul::HANEUL;
    use haneul::transfer;
    use haneul::tx_context::{Self, TxContext};

    public struct A has key { id: UID, balance: Balance<HANEUL> }
    public struct B has key { id: UID, balance: Balance<HANEUL> }

    public fun deposit(c: Coin<HANEUL>, ctx: &mut TxContext) {
        let a = A { id: object::new(ctx), balance: coin::into_balance(c) };
        transfer::transfer(a, tx_context::sender(ctx))
    }

    public fun a_to_b(a: A, ctx: &TxContext) {
        let A { id, balance } = a;
        transfer::transfer(B { id, balance }, tx_context::sender(ctx))
    }

    public fun b_to_a_split(b: B, amount: u64, ctx: &mut TxContext) {
        let B { id, mut balance } = b;
        let c = coin::from_balance(balance.split(amount), ctx);
        transfer::public_transfer(c, tx_context::sender(ctx));
        transfer::transfer(A { id, balance }, tx_context::sender(ctx))
    }
}

//# programmable --inputs 1000 --sender A
//> 0: SplitCoins(Gas, [Input(0)]);
//> ex::m::deposit(Result(0))

//# view-object 2,0

//# run ex::m::a_to_b --args object(2,0) --sender A

//# view-object 2,0

//# run ex::m::b_to_a_split --args object(2,0) 400 --sender A

//# view-object 2,0

//# view-object 6,0
