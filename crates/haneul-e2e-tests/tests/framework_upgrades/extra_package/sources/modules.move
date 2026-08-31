// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module haneul_extra::msim_extra_1;

public struct S has key { id: UID }

fun init(ctx: &mut TxContext) {
    transfer::share_object(S {
        id: object::new(ctx),
    })
}

public fun canary(): u64 { 43 }
