// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module move_test_code::init_with_event;

public struct Event has copy, drop {}

fun init(_ctx: &mut TxContext) {
    haneul::event::emit(Event {});
}
