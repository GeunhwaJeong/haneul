// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module haneul::versioned_tests;

use haneul::versioned;

#[test]
fun test_upgrade() {
    let mut ctx = tx_context::dummy();
    let mut wrapper = versioned::create(1, 1000u64, &mut ctx);
    assert!(versioned::version(&wrapper) == 1);
    assert!(versioned::load_value(&wrapper) == &1000u64);
    let (old, cap) = versioned::remove_value_for_upgrade(&mut wrapper);
    assert!(old == 1000u64);
    versioned::upgrade(&mut wrapper, 2, 2000u64, cap);
    assert!(versioned::version(&wrapper) == 2);
    assert!(versioned::load_value(&wrapper) == &2000u64);
    let value = versioned::destroy(wrapper);
    assert!(value == 2000u64);
}
