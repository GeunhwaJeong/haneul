// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests matching on unit type ()
module 0x42::m;

fun test(): u64 {
    let x = ();
    match (x) {
        () => 42,
    }
}
