// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module emit_event::emit_event {
    use haneul::event;
    use std::ascii;

    public struct TestEvent has copy, drop {
        message: ascii::String,
        value: u64,
    }

    // This init function will automatically run when the package is published
    // and will emit an event that we can test for
    fun init(_ctx: &mut TxContext) {
        event::emit(TestEvent {
            message: ascii::string(b"Package published successfully!"),
            value: 42,
        });
    }
}
