// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

module haneul::hmac;

/// @param key: HMAC key, arbitrary bytes.
/// @param msg: message to sign, arbitrary bytes.
/// Returns the 32 bytes digest of HMAC-SHA3-256(key, msg).
public native fun hmac_sha3_256(key: &vector<u8>, msg: &vector<u8>): vector<u8>;
