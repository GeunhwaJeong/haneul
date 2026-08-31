// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module upgrades::upgrades {

    // created new struct
    public struct NewStruct {
        new_field: u64
    }

    // created new enum
    public enum NewEnum {
        A,
    }

    // created new function
    fun new_function(): u64 {
        0
    }
    
}