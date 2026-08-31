// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_macros::sim_test;
use std::path::PathBuf;
use std::time::Duration;

#[sim_test]
async fn smoke_test() {
    // This test makes sure that the haneul surfer runs.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "move_building_blocks"]);
    let results =
        haneul_surfer::run(Duration::from_secs(30), Duration::from_secs(15), vec![path]).await;
    assert!(results.num_successful_transactions > 0);
    assert!(!results.unique_move_functions_called.is_empty());
}
