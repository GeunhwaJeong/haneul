// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//# init --addresses ex=0x0

//# publish
module ex::m;

public struct Priv has key {
    id: UID,
}

public fun mint(ctx: &mut TxContext) {
    let q = Priv { id: object::new(ctx) };
    transfer::transfer(q, ctx.sender());
}

public fun priv(ctx: &mut TxContext): Priv {
    Priv { id: object::new(ctx) }
}

//# run ex::m::mint

// Does not have store
//# programmable --inputs object(2,0) @0
//> 0: haneul::party::single_owner(Input(1));
//> haneul::transfer::public_party_transfer<ex::m::Priv>(Input(0), Result(0))

// Does not have store
//# programmable --inputs @0
//> 0: ex::m::priv();
//> 1: haneul::party::single_owner(Input(0));
//> haneul::transfer::public_party_transfer<ex::m::Priv>(Result(0), Result(1))

// Private transfer
//# programmable --inputs object(2,0) @0
//> 0: haneul::party::single_owner(Input(1));
//> haneul::transfer::party_transfer<ex::m::Priv>(Input(0), Result(0))

// Private transfer
//# programmable --inputs @0
//> 0: ex::m::priv();
//> 1: haneul::party::single_owner(Input(0));
//> haneul::transfer::party_transfer<ex::m::Priv>(Result(0), Result(1))
