// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module loops::infinite_loop;

public fun inf_loop_0() { loop { continue } }
public fun inf_loop_1() { while (true) { continue } }
