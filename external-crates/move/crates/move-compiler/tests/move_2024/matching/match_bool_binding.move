// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests matching on bool with binder (wildcard-like) alongside literal
module 0x42::m;

fun test(b: bool): u64 {
    match (b) {
        true => 1,
        x => if (x) { 2 } else { 0 },
    }
}
