// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { KioskListing, KioskOwnerCap } from '@haneullabs/kiosk';
import { HaneulObjectResponse } from '@haneullabs/haneul/client';
import { GEUNHWA_PER_HANEUL, normalizeHaneulAddress } from '@haneullabs/haneul/utils';

// Parse the display of a list of objects into a simple {object_id: display} map
// to use throughout the app.
export const parseObjectDisplays = (
	data: HaneulObjectResponse[],
): Record<string, Record<string, string> | undefined> => {
	return data.reduce<Record<string, Record<string, string> | undefined>>(
		(acc, item: HaneulObjectResponse) => {
			const display = item.data?.display?.data;
			const id = item.data?.objectId!;
			acc[id] = display || undefined;
			return acc;
		},
		{},
	);
};

export const processKioskListings = (data: KioskListing[]): Record<string, KioskListing> => {
	const results: Record<string, KioskListing> = {};

	data
		.filter((x) => !!x)
		.map((x: KioskListing) => {
			results[x.objectId || ''] = x;
			return x;
		});
	return results;
};

export const geunhwaToHaneul = (geunhwa: bigint | string | undefined) => {
	if (!geunhwa) return 0;
	return Number(geunhwa || 0) / Number(GEUNHWA_PER_HANEUL);
};

export const formatHaneul = (amount: number) => {
	return new Intl.NumberFormat('en-US', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 5,
	}).format(amount);
};

/**
 * Finds an active owner cap for a kioskId based on the
 * address owned kiosks.
 */
export const findActiveCap = (
	caps: KioskOwnerCap[] = [],
	kioskId: string,
): KioskOwnerCap | undefined => {
	return caps.find((x) => normalizeHaneulAddress(x.kioskId) === normalizeHaneulAddress(kioskId));
};
