// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module bridged_btc::btc {
    use std::option;

    use haneul::coin;
    use haneul::transfer;
    use haneul::tx_context;
    use haneul::tx_context::TxContext;

    struct BTC has drop {}

    const DECIMAL: u8 = 8;

    fun init(otw: BTC, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency(
            otw,
            DECIMAL,
            b"BTC",
            b"Bitcoin",
            b"Bridged Bitcoin token",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
