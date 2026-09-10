// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// A generic reference parameter whose type parameter unifies to TxContext is
// auto-injected like a concrete TxContext parameter, so zero-argument calls
// succeed. A non-TxContext instantiation takes the normal user-argument path.

//# init --addresses test=0x0

//# publish
module test::m;

public fun gen_mut<T>(_: &mut T) {
}

public fun gen_imm<T>(_: &T) {
}

//# programmable
// &mut T unifies to &mut TxContext; the slot is auto-injected
//> test::m::gen_mut<haneul::tx_context::TxContext>();

//# programmable
// &T unifies to &TxContext; the slot is auto-injected
//> test::m::gen_imm<haneul::tx_context::TxContext>();

//# programmable --inputs 0
// T = u64 is not TxContext: no injection, the argument is supplied normally
//> test::m::gen_imm<u64>(Input(0));
