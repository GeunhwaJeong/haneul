// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::protocol_config_tests;

use haneul::protocol_config;

#[test]
fun test_is_feature_enabled_true() {
    let is_enabled = protocol_config::is_feature_enabled(b"advance_epoch_start_time_in_safe_mode");
    assert!(is_enabled, 1);
}

#[test]
fun test_is_feature_enabled_false() {
    let is_enabled = protocol_config::is_feature_enabled(
        b"per_command_shared_object_transfer_rules",
    );
    assert!(!is_enabled, 1);
}
