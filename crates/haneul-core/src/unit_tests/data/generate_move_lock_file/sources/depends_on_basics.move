// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

/// Create a dependency on a package to test Move.lock generation.
module depends::depends_on_basics;

use examples::object_basics;

public fun delegate(ctx: &mut TxContext) {
    object_basics::share(ctx);
}
