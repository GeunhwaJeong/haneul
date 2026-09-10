// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// docs::#withdraw
import { Transaction } from '@haneullabs/haneul/transactions';
import { Ed25519Keypair } from '@haneullabs/haneul/keypairs/ed25519';
import { client } from './client.js';
import { PREDICT } from './config.js';

export async function withdrawLiquidity(params: {
	signer: Ed25519Keypair;
	plpCoinId: string;
}) {
	const { signer, plpCoinId } = params;
	const tx = new Transaction();

	const quote = tx.moveCall({
		target: `${PREDICT.packageId}::predict::withdraw`,
		typeArguments: [PREDICT.quoteType],
		arguments: [tx.object(PREDICT.predictObjectId), tx.object(plpCoinId), tx.object.clock()],
	});
	tx.transferObjects([quote], signer.toHaneulAddress());

	const result = await client.signAndExecuteTransaction({
		transaction: tx,
		signer,
		include: { effects: true },
	});
	// Wait for finality before acting on the result, so later reads reflect it.
	await client.waitForTransaction({ result });

	if (result.$kind === 'FailedTransaction') {
		// The transaction is onchain and the sender paid gas. Do not retry it.
		const { status } = result.FailedTransaction;
		throw new Error(
			`withdraw aborted: ${status.success ? 'unknown' : JSON.stringify(status.error)}`,
		);
	}
	return result.Transaction;
}
// docs::/#withdraw
