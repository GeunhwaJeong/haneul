// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// docs::#client
import { HaneulGrpcClient } from '@haneullabs/haneul/grpc';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';
import { decodeHaneulPrivateKey } from '@haneullabs/haneul/cryptography';
import {
	deepbook,
	type DeepBookClient,
	type MarginManager,
	type BalanceManager,
} from '@haneullabs/deepbook-v3';
import type { ClientWithExtensions } from '@haneullabs/haneul/client';

export type DeepBookMarginClient = ClientWithExtensions<{ deepbook: DeepBookClient }>;

export function getKeypair(privateKey: string): Ed25519Keypair {
	const { secretKey } = decodeHaneulPrivateKey(privateKey);
	return Ed25519Keypair.fromSecretKey(secretKey);
}

// Testnet DeepBook client with margin enabled. On Testnet the SDK auto-loads the
// margin package IDs, margin pools, and Pyth config, so you reference pools,
// coins, and managers by key. Read-only calls (risk parameters, pool liquidity)
// work without a manager; borrowing and trading through a margin manager need
// `marginManagers`. Supplying to a margin pool and staking use a spot
// `BalanceManager`, so pass `balanceManagers` when you compose those legs.
export function marginClient(
	address: string,
	options?: {
		marginManagers?: { [key: string]: MarginManager };
		balanceManagers?: { [key: string]: BalanceManager };
	},
): DeepBookMarginClient {
	return new HaneulGrpcClient({
		network: 'testnet',
		baseUrl: 'https://fullnode.testnet.haneul.io:443',
	}).$extend(deepbook({ address, ...options }));
}
// docs::/#client
