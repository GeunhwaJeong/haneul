// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests at-pattern wrapping a constant pattern
module 0x42::m;

const C: u64 = 42;

fun test(x: u64): u64 {
    match (x) {
        y @ C => y,
        _ => 0,
    }
}
