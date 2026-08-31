// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// docs::#keeper
import { Transaction } from '@haneullabs/haneul/transactions';
import type { HaneulClient } from '@haneullabs/haneul/client';
import type { Signer } from '@haneullabs/haneul/cryptography';
import type { HaneulPythClient, HaneulPriceServiceConnection } from '@pythnetwork/pyth-haneul-js';

// One push cycle: fetch the latest signed update from Hermes and apply it
// onchain, refreshing the feed's PriceInfoObject. A pull consumer updates and
// reads in its own transaction; a push consumer instead relies on a keeper
// running this loop so the stored price is recent when the consumer reads it.
//
// Each cycle is a transaction that costs gas plus the Pyth base update fee, so
// choose an interval that balances freshness against cost. A push consumer must
// still check the stored price's age, because the keeper can fall behind.
export async function pushOnce(
	haneul: HaneulClient,
	pyth: HaneulPythClient,
	hermes: HaneulPriceServiceConnection,
	signer: Signer,
	feedId: string,
): Promise<string> {
	const updates = await hermes.getPriceFeedsUpdateData([feedId]);
	const tx = new Transaction();
	await pyth.updatePriceFeeds(tx, updates, [feedId]);
	tx.setGasBudget(150_000_000n);
	const r = await haneul.signAndExecuteTransaction({
		transaction: tx,
		signer,
		options: { showEffects: true },
	});
	return r.digest;
}

// Keeper loop: push on an interval, and back off exponentially on failure so a
// transient RPC or Hermes error does not hammer the network or burn gas. A
// production keeper also caps total spend, alerts on repeated failures, and
// keeps its signing key off the hot path.
export async function runKeeper(
	push: () => Promise<string>,
	intervalMs: number,
	maxBackoffMs: number,
): Promise<void> {
	let backoff = intervalMs;
	for (;;) {
		try {
			const digest = await push();
			console.log('pushed', digest);
			backoff = intervalMs;
		} catch (e) {
			console.error('push failed, backing off:', String(e).slice(0, 120));
			backoff = Math.min(backoff * 2, maxBackoffMs);
		}
		await new Promise((r) => setTimeout(r, backoff));
	}
}
// docs::/#keeper
