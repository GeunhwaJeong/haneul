// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@haneullabs/haneul/transactions';
import { HaneulGrpcClient } from '@haneullabs/haneul/grpc';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';

const client = new HaneulGrpcClient({ baseUrl: 'https://fullnode.testnet.haneul.io:443', network: 'testnet' });
const keypair = new Ed25519Keypair();

// docs::#multi-recipient
const tx = new Transaction();

const recipients = [
	{ address: '0xAlice...', amount: 1_000_000_000n }, // 1 HANEUL
	{ address: '0xBob...', amount: 500_000_000n }, // 0.5 HANEUL
	{ address: '0xCarol...', amount: 250_000_000n }, // 0.25 HANEUL
];

// Split the gas coin into one coin per recipient.
const coins = tx.splitCoins(
	tx.gas,
	recipients.map((r) => tx.pure('u64', r.amount)),
);

// Transfer each split coin to its recipient.
recipients.forEach((r, i) => {
	tx.transferObjects([coins[i]], tx.pure('address', r.address));
});

await client.signAndExecuteTransaction({ signer: keypair, transaction: tx });
// docs::/#multi-recipient
