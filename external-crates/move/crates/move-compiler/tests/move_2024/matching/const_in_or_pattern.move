// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests constant mixed with literal in or-pattern
module 0x42::m;

const C: u64 = 42;

fun test(x: u64): u64 {
    match (x) {
        0 | C => 1,
        _ => 2,
    }
}
