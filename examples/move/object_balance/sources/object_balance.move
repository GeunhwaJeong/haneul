// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module object_balance::object_balance;

use haneul::balance::Balance;

public struct Vault has key {
    id: UID,
}

public fun create(ctx: &mut TxContext): Vault {
    Vault {
        id: object::new(ctx),
    }
}

public fun new_owned(ctx: &mut TxContext) {
    let vault = create(ctx);
    transfer::transfer(vault, ctx.sender());
}

public fun new_party(ctx: &mut TxContext) {
    let vault = create(ctx);
    transfer::party_transfer(vault, haneul::party::single_owner(ctx.sender()));
}

public fun new_shared(ctx: &mut TxContext) {
    let vault = create(ctx);
    transfer::share_object(vault);
}

public fun withdraw_funds<T>(vault: &mut Vault, amount: u64): Balance<T> {
    haneul::balance::redeem_funds(
        haneul::balance::withdraw_funds_from_object(
            &mut vault.id,
            amount,
        ),
    )
}
