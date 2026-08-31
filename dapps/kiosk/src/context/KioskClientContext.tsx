// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClient, useHaneulClientContext } from '@haneullabs/dapp-kit';
import { KioskClient, Network } from '@haneullabs/kiosk';
import { createContext, ReactNode, useContext, useMemo } from 'react';

export const KioskClientContext = createContext<KioskClient | undefined>(undefined);

export function KioskClientProvider({ children }: { children: ReactNode }) {
	const haneulClient = useHaneulClient();
	const { network } = useHaneulClientContext();
	const kioskClient = useMemo(
		() =>
			new KioskClient({
				client: haneulClient,
				network: network as Network,
			}),
		[haneulClient, network],
	);

	return <KioskClientContext.Provider value={kioskClient}>{children}</KioskClientContext.Provider>;
}

export function useKioskClient() {
	const kioskClient = useContext(KioskClientContext);
	if (!kioskClient) {
		throw new Error('kioskClient not setup properly.');
	}
	return kioskClient;
}
