// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module overlapping_summaries::b;

#[allow(unused)]
public struct Y {
    x: child_pkg::a::X,
    y: other_child::a::X,
}

public fun g() {}
