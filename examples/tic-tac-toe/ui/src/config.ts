// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { createNetworkConfig } from "@haneullabs/dapp-kit";
import { getFullnodeUrl } from "@haneullabs/haneul/client";

import DevnetPackage from "./env.devnet.ts";
import LocalnetPackage from "./env.localnet.ts";
import MainnetPackage from "./env.mainnet.ts";
import TestnetPackage from "./env.testnet.ts";

const { networkConfig, useNetworkVariable } = createNetworkConfig({
    localnet: {
        url: getFullnodeUrl("localnet"),
        variables: {
            explorer: (id: string) => `https://explorer.polymedia.app/object/${id}/?network=local`,
            ...LocalnetPackage,
        },
    },
    devnet: {
        url: getFullnodeUrl("devnet"),
        variables: {
            explorer: (id: string) => `https://haneulscan.xyz/devnet/object/${id}/`,
            ...DevnetPackage,
        },
    },
    testnet: {
        url: getFullnodeUrl("testnet"),
        variables: {
            explorer: (id: string) => `https://haneulscan.xyz/testnet/object/${id}/`,
            ...TestnetPackage,
        },
    },
    mainnet: {
        url: getFullnodeUrl("mainnet"),
        variables: {
            explorer: (id: string) => `https://haneulscan.xyz/mainnet/object/${id}/`,
            ...MainnetPackage,
        },
    },
});

export { networkConfig, useNetworkVariable };
