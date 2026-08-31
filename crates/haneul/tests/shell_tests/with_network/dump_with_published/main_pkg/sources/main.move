// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module main_pkg::main;

use dep_pkg::dep;

public fun call_dep(): u64 {
    dep::hello()
}
