// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module deepbook_client::deepbook_client {
    use deepbook::clob::Order;

    public fun f(): Order {
        abort(0)
    }
}
