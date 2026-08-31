// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_replay::ReplayToolCommand;
use haneul_replay::execute_replay_command;
use std::path::PathBuf;

#[tokio::test]
async fn replay_sandboxes() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/sandbox_snapshots");

    // For each file in the sandbox, replay the transactions and compare the results.
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        assert!(path.is_file());
        let cmd = ReplayToolCommand::ReplaySandbox { path };

        execute_replay_command(None, true, true, None, None, cmd)
            .await
            .unwrap();
    }
}
