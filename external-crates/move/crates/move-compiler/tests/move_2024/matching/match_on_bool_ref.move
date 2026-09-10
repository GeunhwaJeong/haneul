// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests matching on a reference to a boolean
module 0x42::m;

fun test(b: &bool): u64 {
    match (b) {
        true => 1,
        false => 0,
    }
}
