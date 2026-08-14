// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// docs::#client
import { HaneulGrpcClient } from '@haneullabs/haneul/grpc';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';
import { decodeHaneulPrivateKey } from '@haneullabs/haneul/cryptography';
import { PREDICT } from './config.js';

export function getKeypair(privateKey: string): Ed25519Keypair {
	const { secretKey } = decodeHaneulPrivateKey(privateKey);
	return Ed25519Keypair.fromSecretKey(secretKey);
}

export const client = new HaneulGrpcClient({
	network: PREDICT.network,
	baseUrl: 'https://fullnode.testnet.haneul.io:443',
});
// docs::/#client
