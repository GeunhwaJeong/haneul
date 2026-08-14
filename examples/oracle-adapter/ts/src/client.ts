// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// docs::#client
import { HaneulClient } from '@haneullabs/haneul/client';
import { HaneulPythClient, HaneulPriceServiceConnection } from '@pythnetwork/pyth-haneul-js';
import { TESTNET } from './config.js';

// A Haneul RPC client, the Pyth on-chain client (which builds the Wormhole verify
// plus price-update commands), and a Hermes connection (which serves the signed
// off-chain price updates). Pyth on Haneul is a pull oracle: you fetch an update
// from Hermes and apply it on-chain in the same transaction that reads it.
export function haneulClient(): HaneulClient {
	return new HaneulClient({ url: TESTNET.rpcUrl });
}

export function pythClient(haneul: HaneulClient): HaneulPythClient {
	return new HaneulPythClient(haneul, TESTNET.pythStateId, TESTNET.wormholeStateId);
}

export function hermes(): HaneulPriceServiceConnection {
	return new HaneulPriceServiceConnection(TESTNET.hermesEndpoint);
}
// docs::/#client
