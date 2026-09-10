// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module 0x0::M {
    public native struct S;

    fun f(s: S): u64 {
        match (s) {
            S {} => 0,
        }
    }

    // by-reference destructuring
    fun f_ref(s: &S): u64 {
        match (s) {
            S {} => 0,
        }
    }

    // by-mut-reference destructuring
    fun f_mut_ref(s: &mut S): u64 {
        match (s) {
            S {} => 0,
        }
    }
}
