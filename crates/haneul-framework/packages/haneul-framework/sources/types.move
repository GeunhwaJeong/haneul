// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

/// Haneul types helpers and utilities
module haneul::types;

// === one-time witness ===

/// Tests if the argument type is a one-time witness, that is a type with only one instantiation
/// across the entire code base.
public native fun is_one_time_witness<T: drop>(_: &T): bool;
