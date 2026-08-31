// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use haneul::dynamic_object_field as ofield;

    public struct S has key, store {
        id: haneul::object::UID,
    }

    public struct R has key {
        id: haneul::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let mut id = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        ofield::add(&mut id, 0u64, child);
        let parent = S { id };
        let r = R { id: haneul::object::new(ctx), s: parent };
        haneul::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
