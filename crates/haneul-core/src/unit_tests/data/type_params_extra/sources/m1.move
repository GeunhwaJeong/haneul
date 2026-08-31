// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module type_params::m1;

public fun transfer_object<T: key + store>(o: T, recipient: address) {
    transfer::public_transfer(o, recipient);
}
