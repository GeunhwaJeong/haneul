// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// SplitCoins and MergeCoins write through their coin argument. The write is allowed through a
// `&mut Coin` result, and once every extension of the coin is dead.

//# init --addresses test=0x0 --accounts A B

//# publish
module test::m;

use haneul::coin::Coin;
use haneul::haneul::HANEUL;

public fun id_mut<T>(t: &mut T): &mut T { t }
public fun use_mut<T>(_: &mut T) {}
public fun amount_ref(_: &Coin<HANEUL>, x: &u64): &u64 { x }

//# programmable --sender A --inputs 1000 @B
// VALID: SplitCoins through a `&mut Coin<HANEUL>` result rooted in the gas coin
//> 0: test::m::id_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(Gas);
//> 1: SplitCoins(Result(0), [Input(0)]);
//> 2: TransferObjects([Result(1)], Input(1));

//# programmable --sender A --inputs 1000 100 @B
// VALID: split through a `&mut` result twice, then through the value
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::m::id_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(Result(0));
//> 2: SplitCoins(Result(1), [Input(1)]);
//> 3: SplitCoins(Result(1), [Input(1)]);
//> 4: SplitCoins(Result(0), [Input(1)]);
//> 5: TransferObjects([Result(0), Result(2), Result(3), Result(4)], Input(2));

//# programmable --sender A --inputs 1000 100 @B
// VALID: merge into a `&mut Coin<HANEUL>` result
//> 0: SplitCoins(Gas, [Input(0), Input(1)]);
//> 1: test::m::id_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(NestedResult(0,0));
//> 2: MergeCoins(Result(1), [NestedResult(0,1)]);
//> 3: TransferObjects([NestedResult(0,0)], Input(2));

//# programmable --sender A --inputs 1000 100 @B
// VALID: the same `&mut Coin<HANEUL>` result remains usable after the split
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::m::id_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(Result(0));
//> 2: SplitCoins(Result(1), [Input(1)]);
//> 3: test::m::use_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(Result(1));
//> 4: TransferObjects([Result(0), Result(2)], Input(2));

//# programmable --sender A --inputs 1000 100 @B
// VALID: the `&mut Balance` extension is dead by the time of the split
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: test::m::id_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(Result(0));
//> 2: haneul::coin::balance_mut<haneul::haneul::HANEUL>(Result(1));
//> 3: test::m::use_mut<haneul::balance::Balance<haneul::haneul::HANEUL>>(Result(2));
//> 4: SplitCoins(Result(1), [Input(1)]);
//> 5: TransferObjects([Result(0), Result(4)], Input(2));

//# programmable --sender A --inputs 1000 100 @B
// VALID: balance operations through the extension, then the coin split once the extension is dead
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::balance_mut<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::split<haneul::haneul::HANEUL>(Result(1), Input(1));
//> 3: haneul::balance::join<haneul::haneul::HANEUL>(Result(1), Result(2));
//> 4: SplitCoins(Result(0), [Input(1)]);
//> 5: TransferObjects([Result(0), Result(4)], Input(2));

//# programmable --sender A --inputs 100 @A
// VALID: an amount read through a reference rooted in the coin is released before the coin is written
//> 0: test::m::amount_ref(Gas, Input(0));
//> 1: SplitCoins(Gas, [Result(0)]);
//> 2: TransferObjects([Result(1)], Input(1));
