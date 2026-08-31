// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { PublicKey } from "@haneullabs/haneul/cryptography";
import { MultiSigPublicKey } from "@haneullabs/haneul/multisig";

/**
 * Generate the public key corresponding to a 1-of-N multi-sig
 * composed of `keys` (all with equal weighting).
 */
export function multiSigPublicKey(keys: PublicKey[]): MultiSigPublicKey {
    // Multi-sig addresses cannot contain the same public keys multiple
    // times. In our case, it's fine to de-duplicate them because all
    // keys get equal weight and the threshold is 1.
    const deduplicated: { [key: string]: PublicKey } = {};
    for (const key of keys) {
        deduplicated[key.toHaneulAddress()] = key;
    }

    return MultiSigPublicKey.fromPublicKeys({
        threshold: 1,
        publicKeys: Object.values(deduplicated).map((publicKey) => {
            return { publicKey, weight: 1 };
        }),
    });
}
