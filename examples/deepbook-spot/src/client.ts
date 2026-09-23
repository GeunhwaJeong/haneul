// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// docs::#client
import { HaneulGrpcClient } from '@haneullabs/haneul/grpc';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';
import { decodeHaneulPrivateKey } from '@haneullabs/haneul/cryptography';
import { deepbook, type DeepBookClient, type BalanceManager } from '@haneullabs/deepbook-v3';
import type { ClientWithExtensions } from '@haneullabs/haneul/client';

export type DeepBookTestnetClient = ClientWithExtensions<{ deepbook: DeepBookClient }>;

export function getKeypair(privateKey: string): Ed25519Keypair {
	const { secretKey } = decodeHaneulPrivateKey(privateKey);
	return Ed25519Keypair.fromSecretKey(secretKey);
}

// Testnet DeepBook client. The SDK ships Testnet package, coin, and pool
// constants, so you reference pools and coins by key (for example 'DEEP_HANEUL' or
// 'DEEP') instead of hardcoding IDs. Read-only calls work without a manager.
export function deepbookClient(
	address: string,
	balanceManagers?: { [key: string]: BalanceManager },
): DeepBookTestnetClient {
	return new HaneulGrpcClient({
		network: 'testnet',
		baseUrl: 'https://fullnode.testnet.haneul.io:443',
	}).$extend(deepbook({ address, balanceManagers }));
}
// docs::/#client
