// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// tests invalid gas coin usage by value

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    public entry fun t1<T>(_: T) {
        abort 0
    }

    public fun t2<T>(_: T) {
        abort 0
    }

    entry fun t3<T>(_: T) {
        abort 0
    }
}

// cannot pass to Move function
//# programmable --sender A
//> test::m1::t1<haneul::coin::Coin<haneul::haneul::HANEUL>>(Gas)

//# programmable --sender A
//> test::m1::t2<haneul::coin::Coin<haneul::haneul::HANEUL>>(Gas)

//# programmable --sender A
//> test::m1::t2<haneul::coin::Coin<haneul::haneul::HANEUL>>(Gas)

// cannot merge gas coin
//# programmable --sender A --inputs 10  --gas-budget 10000000000
//> 0: SplitCoins(Gas, [Input(0)]);
//> MergeCoins(Result(0), [Gas])

// cannot use gas coin in a vector
//# programmable --sender A
//> MakeMoveVec([Gas])

// we give the error that the gas coin was taken, even though this call is invalid
//# programmable --sender A --inputs @A
//> TransferObjects([Gas], Input(0));
//> test::m1::t1<haneul::coin::Coin<haneul::haneul::HANEUL>>(Gas)

//# programmable --sender A --inputs @A
// Similarly we try to use the gas coin both with send_funds and TransferObjects, and get an
// error that the gas coin was already moved, even though the call is _valid_.
//> TransferObjects([Gas], Input(0));
//> haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0))
