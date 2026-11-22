// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module move_test_code_clock::clock_test;

use haneul::clock::{Self, Clock};
use haneul::event;

const EAbortAfterRead: u64 = 7;

public struct BorrowedTimestamp has copy, drop {
    timestamp_ms: u64,
}

/// A function written against `&Clock`, as existing code is.
fun timestamp_of(clock: &Clock): u64 {
    clock.timestamp_ms()
}

public entry fun emit_borrowed_timestamp() {
    event::emit(BorrowedTimestamp { timestamp_ms: timestamp_of(clock::borrow()) });
}

public entry fun assert_matches_clock(clock: &Clock) {
    assert!(clock::borrow().timestamp_ms() == clock.timestamp_ms(), 0);
}

public entry fun read_then_abort() {
    let _ = clock::borrow().timestamp_ms();
    abort EAbortAfterRead
}
