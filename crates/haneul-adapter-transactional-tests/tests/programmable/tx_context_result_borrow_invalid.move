// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// tests invalid attempts at manually supplying &mut/&TxContext

//# init --addresses test=0x0 --enable-feature-flags allow_references_in_ptbs

//# publish
module test::m;

public fun mut_id(ctx: &mut TxContext): &mut TxContext {
    ctx
}

public fun imm_id(ctx: &TxContext): &TxContext {
    ctx
}

public fun mut_tx(_: &mut TxContext) {
}

public fun imm_tx(_: &TxContext) {
}

public fun imm_imm_tx(_: &TxContext, _: &TxContext) {
}

public fun gen_mut<T>(_: &mut T) {
}

public fun gen_imm<T>(_: &T) {
}

public fun gen_mut_tx<T>(_: &mut T, _: &mut TxContext) {
}

public fun gen_imm_tx<T>(_: &T, _: &TxContext) {
}

//# programmable
// cannot manually supply the TxContext
//> 0: test::m::mut_id();
//> test::m::mut_tx(Result(0));

//# programmable
// cannot manually supply the TxContext
//> 0: test::m::mut_id();
//> test::m::imm_tx(Result(0));

//# programmable
// cannot manually supply the TxContext
//> 0: test::m::imm_id();
//> test::m::imm_tx(Result(0));

//# programmable
// cannot manually supply the TxContext, even when one is inferred
//> 0: test::m::imm_id();
//> test::m::imm_imm_tx(Result(0));

//# programmable
// cannot manually supply the TxContext, even when generic
//> 0: test::m::mut_id();
//> test::m::gen_mut<haneul::tx_context::TxContext>(Result(0));

//# programmable
// cannot manually supply the TxContext, even when generic
//> 0: test::m::mut_id();
//> test::m::gen_imm<haneul::tx_context::TxContext>(Result(0));

//# programmable
// cannot manually supply the TxContext, even when generic
//> 0: test::m::imm_id();
//> test::m::gen_imm<haneul::tx_context::TxContext>(Result(0));

//# programmable
// cannot manually supply the TxContext, even when multiple supplied
//> 0: test::m::mut_id();
//> test::m::gen_mut_tx<haneul::tx_context::TxContext>(Result(0),Result(0));

//# programmable
// cannot manually supply the TxContext, even when multiple supplied
//> 0: test::m::mut_id();
//> test::m::gen_imm_tx<haneul::tx_context::TxContext>(Result(0),Result(0));

//# programmable
// cannot manually supply the TxContext, even when multiple supplied
//> 0: test::m::imm_id();
//> test::m::gen_imm_tx<haneul::tx_context::TxContext>(Result(0),Result(0));

//# programmable
// cannot manually supply the TxContext, even when generic and inferred
//> 0: test::m::mut_id();
//> test::m::gen_mut_tx<haneul::tx_context::TxContext>(Result(0));

//# programmable
// cannot manually supply the TxContext,  even when generic and inferred
//> 0: test::m::mut_id();
//> test::m::gen_imm_tx<haneul::tx_context::TxContext>(Result(0));

//# programmable
// cannot manually supply the TxContext,  even when generic and inferred
//> 0: test::m::imm_id();
//> test::m::gen_imm_tx<haneul::tx_context::TxContext>(Result(0));
