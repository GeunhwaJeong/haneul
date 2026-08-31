// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    public struct S has key, store {
        id: haneul::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let mut parent = haneul::object::new(ctx);
        let child = S { id: haneul::object::new(ctx) };
        haneul::dynamic_object_field::add(&mut parent, 0u64, child);
        haneul::object::delete(parent);
    }
}

//# run test::m::t --sender A
