// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { useSignTransaction, useHaneulClient } from '@haneullabs/dapp-kit';
import { HaneulTransactionBlockResponseOptions } from '@haneullabs/haneul/client';
import { Transaction } from '@haneullabs/haneul/transactions';

// A helper to execute transactions by:
// 1. Signing them using the wallet
// 2. Executing them using the rpc provider
export function useTransactionExecution() {
	const provider = useHaneulClient();

	// sign transaction from the wallet
	const { mutateAsync: signTransaction } = useSignTransaction();

	// tx: Transaction
	const signAndExecute = async ({
		tx,
		options = { showEffects: true },
	}: {
		tx: Transaction;
		options?: HaneulTransactionBlockResponseOptions | undefined;
	}) => {
		const signedTx = await signTransaction({ transaction: tx });

		const res = await provider.executeTransactionBlock({
			transactionBlock: signedTx.bytes,
			signature: signedTx.signature,
			options,
		});

		const status = res.effects?.status?.status === 'success';

		if (status) return true;
		else throw new Error('Transaction execution failed.');
	};

	return { signAndExecute };
}
