// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

/// Coin<HANEUL> is the token used to pay for gas in Haneul.
/// It has 9 decimals, and the smallest unit (10^-9) is called "geunhwa".
module haneul::haneul;

use haneul::balance::Balance;
use haneul::coin;

const EAlreadyMinted: u64 = 0;
/// Sender is not @0x0 the system address.
const ENotSystemAddress: u64 = 1;

#[allow(unused_const)]
/// The amount of Geunhwa per Haneul token based on the fact that geunhwa is
/// 10^-9 of a Haneul token
const GEUNHWA_PER_HANEUL: u64 = 1_000_000_000;

#[allow(unused_const)]
/// The total supply of Haneul denominated in whole Haneul tokens (10 Billion)
const TOTAL_SUPPLY_HANEUL: u64 = 10_000_000_000;

/// The total supply of Haneul denominated in Geunhwa (10 Billion * 10^9)
const TOTAL_SUPPLY_GEUNHWA: u64 = 10_000_000_000_000_000_000;

/// Name of the coin
public struct HANEUL has drop {}

#[allow(unused_function, deprecated_usage)]
/// Register the `HANEUL` Coin to acquire its `Supply`.
/// This should be called only once during genesis creation.
fun new(ctx: &mut TxContext): Balance<HANEUL> {
    assert!(ctx.sender() == @0x0, ENotSystemAddress);
    assert!(ctx.epoch() == 0, EAlreadyMinted);

    let (treasury, metadata) = coin::create_currency(
        HANEUL {},
        9,
        b"HANEUL",
        b"Haneul",
        // TODO: add appropriate description and logo url
        b"",
        option::none(),
        ctx,
    );
    transfer::public_freeze_object(metadata);
    let mut supply = treasury.treasury_into_supply();
    let total_haneul = supply.increase_supply(TOTAL_SUPPLY_GEUNHWA);
    let _ = supply.destroy_supply();
    total_haneul
}

#[allow(lint(public_entry))]
public entry fun transfer(c: coin::Coin<HANEUL>, recipient: address) {
    transfer::public_transfer(c, recipient)
}
