// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@haneullabs/haneul/transactions';
import { HaneulGrpcClient } from '@haneullabs/haneul/grpc';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';

const client = new HaneulGrpcClient({ baseUrl: 'https://fullnode.testnet.haneul.io:443', network: 'testnet' });
const keypair = new Ed25519Keypair();
const playerAddress = '0xPlayer...';

// docs::#result-chaining
const tx = new Transaction();

// Step 1: Create a new game character.
const [character] = tx.moveCall({
	target: '0xGAME::character::create',
	arguments: [tx.pure('string', 'Hero')],
});

// Step 2: Mint a sword and equip it to the character.
const [sword] = tx.moveCall({
	target: '0xGAME::items::mint_sword',
	arguments: [tx.pure('u64', 100)], // attack power
});

tx.moveCall({
	target: '0xGAME::character::equip',
	arguments: [character, sword],
});

// Step 3: Transfer the character to the player.
tx.transferObjects([character], tx.pure.address(playerAddress));

await client.signAndExecuteTransaction({ signer: keypair, transaction: tx });
// docs::/#result-chaining
