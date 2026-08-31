// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import '@haneullabs/dapp-kit/dist/index.css';
import './index.css';
import '@fontsource-variable/inter';
import '@fontsource-variable/red-hat-mono';

import { HaneulClientProvider, WalletProvider } from '@haneullabs/dapp-kit';
import { getFullnodeUrl } from '@haneullabs/haneul/client';
import { QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { queryClient } from './lib/queryClient';
import { router } from './routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<HaneulClientProvider
				defaultNetwork="haneul:mainnet"
				networks={{
					'haneul:testnet': { url: getFullnodeUrl('testnet') },
					'haneul:mainnet': { url: getFullnodeUrl('mainnet') },
					'haneul:devnet': { url: getFullnodeUrl('devnet') },
				}}
			>
				<WalletProvider>
					<RouterProvider router={router} />
				</WalletProvider>
			</HaneulClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
