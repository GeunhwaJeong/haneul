// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// A location may be moved once every reference into it is dead, for every kind of location a
// reference can be rooted in and every command that consumes a value.

//# init --addresses test=0x0 q=0x0 q_2=0x0 --accounts A B

//# publish
module test::m;

use haneul::balance::Balance;
use haneul::coin::Coin;
use haneul::funds_accumulator::Withdrawal;
use haneul::haneul::HANEUL;
use haneul::transfer::Receiving;

public struct Obj has key, store { id: UID, inner: Inner }
public struct Inner has store, copy, drop { f: u64, g: u64 }
public struct NoCopy has drop { f: u64 }
public struct Potato { v: u64 }
public struct Parent has key, store { id: UID }
public struct Child has key, store { id: UID }

public fun new(ctx: &mut TxContext): Obj {
    Obj { id: object::new(ctx), inner: Inner { f: 0, g: 0 } }
}
public fun share(ctx: &mut TxContext) {
    transfer::public_share_object(Obj { id: object::new(ctx), inner: Inner { f: 0, g: 0 } })
}
public fun new_parent(ctx: &mut TxContext): Parent { Parent { id: object::new(ctx) } }
public fun new_child(p: &Parent, ctx: &mut TxContext) {
    transfer::public_transfer(Child { id: object::new(ctx) }, object::id_address(p))
}
public fun inner_mut(o: &mut Obj): &mut Inner { &mut o.inner }
public fun f_mut(i: &mut Inner): &mut u64 { &mut i.f }
public fun nc(): NoCopy { NoCopy { f: 0 } }
public fun nc_f_mut(n: &mut NoCopy): &mut u64 { &mut n.f }
public fun nc_f(n: &NoCopy): &u64 { &n.f }
public fun take(_: NoCopy) {}
public fun read_then_take(x: u64, n: NoCopy) { assert!(x == n.f, 0) }
public fun take_then_mut(o: Obj, _: &mut Inner) { delete(o) }
public fun open(o: &mut Obj): (Potato, &mut Inner) { (Potato { v: 1 }, &mut o.inner) }
public fun close_and_delete(p: Potato, o: Obj) { let Potato { v: _ } = p; delete(o) }
public fun recv_mut(r: &mut Receiving<Child>): &mut Receiving<Child> { r }
public fun receive(p: &mut Parent, r: Receiving<Child>): Child {
    transfer::public_receive(&mut p.id, r)
}
public fun id<T>(t: &T): &T { t }
public fun id_mut<T>(t: &mut T): &mut T { t }
public fun use_mut<T>(_: &mut T) {}
public fun write(r: &mut u64, v: u64) { *r = v }
public fun value_at_least(c: &Coin<HANEUL>, v: u64) { assert!(c.value() >= v, 0) }
public fun limit_at_least(w: &Withdrawal<Balance<HANEUL>>, v: u256) {
    assert!(w.withdrawal_limit() >= v, 0)
}
public fun delete(o: Obj) { let Obj { id, inner: _ } = o; object::delete(id) }
public fun delete_child(c: Child) { let Child { id } = c; object::delete(id) }

//# publish --upgradeable --sender A
module q::m {
    public fun x(): u64 { 0 }
}

//# stage-package
module q_2::m {
    public fun x(): u64 { 1 }
}

//# programmable --sender A --inputs @A
//> 0: test::m::new();
//> 1: TransferObjects([Result(0)], Input(0));

//# programmable --sender A --inputs @A
//> 0: test::m::new();
//> 1: TransferObjects([Result(0)], Input(0));

//# programmable --sender A
//> 0: test::m::share();

//# programmable --sender A --inputs @A
//> 0: test::m::new_parent();
//> 1: TransferObjects([Result(0)], Input(0));

//# programmable --sender A --inputs object(7,0)
//> 0: test::m::new_child(Input(0));

//# programmable --sender A --inputs 1000 @A
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: haneul::coin::into_balance<haneul::haneul::HANEUL>(Result(0));
//> 2: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(1), Input(1));

//# create-checkpoint

// --- values and results ---

//# programmable --inputs 1
// VALID: a result moved once the reference into it is dead
//> 0: test::m::nc();
//> 1: test::m::nc_f_mut(Result(0));
//> 2: test::m::write(Result(1), Input(0));
//> 3: test::m::take(Result(0));

//# programmable --inputs 1
// VALID: a last-use read releases the reference before the root is moved in the same call
//> 0: test::m::nc();
//> 1: test::m::nc_f(Result(0));
//> 2: test::m::read_then_take(Result(1), Result(0));

//# programmable --sender A --inputs object(4,0)
// VALID: an object by value next to a reference into a different object
//> 0: test::m::new();
//> 1: test::m::inner_mut(Result(0));
//> 2: test::m::take_then_mut(Input(0), Result(1));
//> 3: test::m::delete(Result(0));

//# programmable --sender A
// VALID: the parent consumed with the hot potato once the sibling reference is dead
//> 0: test::m::new();
//> 1: test::m::open(Result(0));
//> 2: test::m::use_mut<test::m::Inner>(NestedResult(1,1));
//> 3: test::m::close_and_delete(NestedResult(1,0), Result(0));

// --- object inputs ---

//# programmable --sender A --inputs object(5,0) 10 @B
// VALID: an owned input transferred after the reference into it is dead
//> 0: test::m::inner_mut(Input(0));
//> 1: test::m::f_mut(Result(0));
//> 2: test::m::write(Result(1), Input(1));
//> 3: TransferObjects([Input(0)], Input(2));

//# programmable --sender A --inputs object(6,0) 8
// VALID: a shared input deleted after the reference into it is dead
//> 0: test::m::inner_mut(Input(0));
//> 1: test::m::f_mut(Result(0));
//> 2: test::m::write(Result(1), Input(1));
//> 3: test::m::delete(Input(0));

//# programmable --sender A --inputs object(7,0) receiving(8,0)
// VALID: a receiving input received after the reference into it is dead
//> 0: test::m::recv_mut(Input(1));
//> 1: test::m::use_mut<haneul::transfer::Receiving<test::m::Child>>(Result(0));
//> 2: test::m::receive(Input(0), Input(1));
//> 3: test::m::delete_child(Result(2));

// --- gas, coins, withdrawals ---

//# programmable --sender A --inputs 1000 100 @A
// VALID: a coin merged away once its `&mut Balance` is dead
//> 0: SplitCoins(Gas, [Input(0), Input(1)]);
//> 1: haneul::coin::balance_mut<haneul::haneul::HANEUL>(NestedResult(0,1));
//> 2: test::m::use_mut<haneul::balance::Balance<haneul::haneul::HANEUL>>(Result(1));
//> 3: MergeCoins(NestedResult(0,0), [NestedResult(0,1)]);
//> 4: TransferObjects([NestedResult(0,0)], Input(2));

//# programmable --sender A --inputs withdraw<haneul::balance::Balance<haneul::haneul::HANEUL>>(500) @B 500u256
// VALID: a withdrawal redeemed by value after the reference into it is dead
//> 0: test::m::id_mut<haneul::funds_accumulator::Withdrawal<haneul::balance::Balance<haneul::haneul::HANEUL>>>(Input(0));
//> 1: test::m::limit_at_least(Result(0), Input(2));
//> 2: haneul::balance::redeem_funds<haneul::haneul::HANEUL>(Input(0));
//> 3: haneul::balance::send_funds<haneul::haneul::HANEUL>(Result(2), Input(1));

// --- upgrade tickets and receipts ---

//# programmable --sender A --inputs object(2,1) 0u8 digest(q_2)
// VALID: Upgrade consumes the ticket after its reference's last use
//> 0: haneul::package::authorize_upgrade(Input(0), Input(1), Input(2));
//> 1: test::m::id<haneul::package::UpgradeTicket>(Result(0));
//> 2: haneul::package::ticket_policy(Result(1));
//> 3: Upgrade(q_2, [haneul, std], q, Result(0));
//> 4: haneul::package::commit_upgrade(Input(0), Result(3));

// --- the gas coin, last because send_funds consumes the sender's gas object ---

//# programmable --sender A --inputs @B 1
// VALID: the gas coin sent by value after the reference into it is dead
//> 0: test::m::id_mut<haneul::coin::Coin<haneul::haneul::HANEUL>>(Gas);
//> 1: test::m::value_at_least(Result(0), Input(1));
//> 2: haneul::coin::send_funds<haneul::haneul::HANEUL>(Gas, Input(0));
