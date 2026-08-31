// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Utilities for type resolution using a Haneul fullnode.
//!
//! This crate provides tools for deserializing Move values to JSON by fetching
//! type layouts from packages via RPC, particularly useful for processing
//! events and objects in indexers and other off-chain applications.

pub mod json_visitor;
pub mod package_store;
