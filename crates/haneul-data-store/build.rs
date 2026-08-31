// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("rpc")
        .from_sdl_file("../haneul-indexer-alt-graphql/schema.graphql")
        .expect("Failed to find GraphQL Schema")
        .as_default()
        .unwrap();
}
