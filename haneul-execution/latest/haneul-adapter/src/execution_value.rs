// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_types::storage::{RuntimeObjectResolver, Storage};

/// Interface with the store necessary to execute a programmable transaction
pub trait ExecutionState: Storage + RuntimeObjectResolver {}

impl<T> ExecutionState for T where T: Storage + RuntimeObjectResolver {}
