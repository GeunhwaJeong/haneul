// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// docs::#bootstrap-swap
import { Transaction } from '@haneullabs/haneul/transactions';
import type { DeepBookTestnetClient } from './client.js';

// Swap HANEUL for DEEP on the DEEP_HANEUL Testnet pool. That pool is whitelisted
// (zero fee) and the swap needs no BalanceManager, so it bootstraps DEEP from
// faucet HANEUL. DEEP is the base and HANEUL the quote, so a quote-for-base swap
// spends HANEUL and returns DEEP. Set `minDeepOut` to a nonzero value (for example
// 99% of getQuantityOut's `baseOut`): with `minOut: 0`, a thin or empty book
// silently returns your HANEUL unfilled instead of reverting.
export function swapHaneulForDeep(
	client: DeepBookTestnetClient,
	haneulAmount: number,
	minDeepOut: number,
	recipient: string,
): Transaction {
	const tx = new Transaction();
	const [deepOut, haneulRemainder, deepFee] = tx.add(
		client.deepbook.deepBook.swapExactQuoteForBase({
			poolKey: 'DEEP_HANEUL',
			amount: haneulAmount,
			deepAmount: 0,
			minOut: minDeepOut,
		}),
	);
	tx.transferObjects([deepOut, haneulRemainder, deepFee], recipient);
	return tx;
}
// docs::/#bootstrap-swap
