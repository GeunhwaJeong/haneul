// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@haneullabs/haneul/transactions';
import type { HaneulClient } from '@haneullabs/haneul/client';
import type { HaneulPythClient, HaneulPriceServiceConnection } from '@pythnetwork/pyth-haneul-js';

// docs::#update-read
// Pull model: fetch the signed update from Hermes, apply it on-chain with the
// Pyth client, then read the freshly updated PriceInfoObject in the SAME
// transaction. `updatePriceFeeds` appends the Wormhole-verify and price-update
// commands and returns the PriceInfoObject IDs; pass the matching one to your
// consumer. Reading through the adapter's `read_and_emit` applies the staleness
// bound, so an update that somehow did not land aborts rather than reading old.
export async function buildUpdateAndRead(
	pyth: HaneulPythClient,
	hermes: HaneulPriceServiceConnection,
	feedId: string,
	consumerPackageId: string,
	maxAgeSecs: number,
): Promise<Transaction> {
	const updates = await hermes.getPriceFeedsUpdateData([feedId]);
	const tx = new Transaction();
	const [priceInfoObjectId] = await pyth.updatePriceFeeds(tx, updates, [feedId]);
	tx.moveCall({
		target: `${consumerPackageId}::demo::read_and_emit`,
		arguments: [tx.object(priceInfoObjectId), tx.object.clock(), tx.pure.u64(maxAgeSecs)],
	});
	return tx;
}
// docs::/#update-read

// docs::#stale-read
// A price that has not been updated recently is stale. Reading it with a tight
// `max_age_secs` aborts with the provider's staleness error rather than
// returning an old value. High-stakes consumers must force this check, not skip
// it: this is `dev-inspect`-able, so you can confirm the rejection without
// spending gas or updating the feed.
export async function checkStale(
	haneul: HaneulClient,
	pyth: HaneulPythClient,
	pythStateId: string,
	feedId: string,
	sender: string,
	maxAgeSecs: number,
): Promise<{ status: string; error?: string }> {
	const pythPackageId = await pyth.getPackageId(pythStateId);
	const priceInfoObjectId = await pyth.getPriceFeedObjectId(feedId);
	if (!priceInfoObjectId) throw new Error(`no PriceInfoObject for feed ${feedId}`);
	const tx = new Transaction();
	tx.moveCall({
		target: `${pythPackageId}::pyth::get_price_no_older_than`,
		arguments: [tx.object(priceInfoObjectId), tx.object.clock(), tx.pure.u64(maxAgeSecs)],
	});
	const r = await haneul.devInspectTransactionBlock({ sender, transactionBlock: tx });
	return { status: r.effects.status.status, error: r.effects.status.error };
}
// docs::/#stale-read
