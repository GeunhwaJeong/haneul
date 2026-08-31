// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_enum_compat_util::*;

use crate::{HaneulMoveStruct, HaneulMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "haneul_move_struct.yaml"]);
    check_enum_compat_order::<HaneulMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "haneul_move_value.yaml"]);
    check_enum_compat_order::<HaneulMoveValue>(path);
}
