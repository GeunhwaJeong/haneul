// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("haneul")
        .from_sdl_file("../haneul-indexer-alt-graphql/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
