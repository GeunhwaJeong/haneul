// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module 0x0::M {
    public native struct S;

    fun f(s: &S): u64 {
        match (s) {
            _ => 0,
        }
    }
}
