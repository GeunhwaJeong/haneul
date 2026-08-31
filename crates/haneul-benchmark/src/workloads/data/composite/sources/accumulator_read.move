// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module basics::accumulator_read;

use haneul::accumulator::AccumulatorRoot;
use haneul::balance;
use haneul::event;
use haneul::haneul::HANEUL;

public struct SettledBalanceEvent has copy, drop {
    value: u64,
}

entry fun read_settled_balance(root: &AccumulatorRoot, addr: address) {
    let value = balance::settled_funds_value<HANEUL>(root, addr);
    event::emit(SettledBalanceEvent { value });
}
