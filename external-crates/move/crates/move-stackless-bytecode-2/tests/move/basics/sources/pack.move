// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module basics::pack {
    public struct S has drop { a: u64, b: u64, c: u64 }

    public fun pack(a: u64, b: u64, c: u64): S { S { a, b, c } }
}
