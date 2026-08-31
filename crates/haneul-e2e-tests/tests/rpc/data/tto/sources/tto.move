// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module tto::M1;

use haneul::coin::Coin;
use haneul::haneul::HANEUL;
use haneul::transfer::Receiving;

public struct A has key, store {
    id: UID,
}

public fun start(coin: Coin<HANEUL>, ctx: &mut TxContext) {
    let a = A { id: object::new(ctx) };
    let a_address = object::id_address(&a);

    transfer::public_transfer(a, ctx.sender());
    transfer::public_transfer(coin, a_address);
}

public fun receive(parent: &mut A, x: Receiving<Coin<HANEUL>>) {
    let coin = transfer::public_receive(&mut parent.id, x);
    transfer::public_transfer(coin, @tto);
}

public fun dont_receive(parent: &mut A, _x: Receiving<Coin<HANEUL>>) {
}
