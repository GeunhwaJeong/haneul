// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@haneullabs/haneul/transactions';
import { HaneulGrpcClient } from '@haneullabs/haneul/grpc';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';

const client = new HaneulGrpcClient({ baseUrl: 'https://fullnode.testnet.haneul.io:443', network: 'testnet' });
const keypair = new Ed25519Keypair();
const recipientAddress = '0xRecipient...';

// docs::#send-from-balance
const tx = new Transaction();

// Send 1 HANEUL to the recipient's address balance. The SDK selects the funding source.
tx.moveCall({
	target: '0x2::balance::send_funds',
	typeArguments: ['0x2::haneul::HANEUL'],
	arguments: [tx.balance({ balance: 1_000_000_000n }), tx.pure.address(recipientAddress)],
});
// docs::/#send-from-balance

// docs::#split-coin-object
const tx2 = new Transaction();

// Split from a specific coin object (not the gas coin).
const [coin] = tx2.splitCoins(tx2.object('0xSpecificCoinId'), [tx2.pure('u64', 500_000_000n)]);
tx2.transferObjects([coin], tx2.pure.address(recipientAddress));
// docs::/#split-coin-object

// docs::#withdraw-redeem
const tx3 = new Transaction();

// Withdraw from address balance and redeem to get a Coin<HANEUL>.
const [redeemed] = tx3.moveCall({
	target: '0x2::coin::redeem_funds',
	typeArguments: ['0x2::haneul::HANEUL'],
	arguments: [tx3.withdrawal({ amount: 1_000_000_000 })],
});

// Pass the coin to a Move function that expects Coin<HANEUL>.
tx3.moveCall({
	target: '0xPACKAGE::module::deposit',
	arguments: [redeemed],
});
// docs::/#withdraw-redeem
