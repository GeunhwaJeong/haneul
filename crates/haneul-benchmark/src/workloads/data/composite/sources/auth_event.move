// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module basics::auth_event;

use haneul::event;

public struct E has copy, drop {
    value: u64,
}

entry fun emit_multiple(start_value: u64, count: u64) {
    let mut i = 0;
    while (i < count) {
        event::emit_authenticated(E { value: start_value + i });
        i = i + 1;
    };
}
