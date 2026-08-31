// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

/// Module for testing command results - simple functions for clear testing
module command_results::test_commands {
    use haneul::coin::{Self, Coin};
    use haneul::haneul::HANEUL;

    public struct TestObject has drop {
        value: u64,
    }

    /// Simple function that takes pure value - demonstrates Input arguments
    public fun create_test_object(value: u64): TestObject {
        TestObject { value }
    }

    /// Function that takes an object - demonstrates Result arguments  
    public fun get_object_value(obj: &TestObject): u64 {
        obj.value
    }

    /// Function that takes gas coin by reference - demonstrates GasCoin arguments
    public fun check_gas_coin(coin: &Coin<HANEUL>): u64 {
        coin::value(coin)
    }

    /// Function that mutates an object by reference - demonstrates mutated_references
    public fun update_object_value(obj: &mut TestObject, new_value: u64) {
        obj.value = new_value;
    }
}
