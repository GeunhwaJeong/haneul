// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { useSignAndExecuteTransaction, useHaneulClient } from "@haneullabs/dapp-kit";
import { HaneulClient, HaneulTransactionBlockResponse } from "@haneullabs/haneul/client";
import { Transaction } from "@haneullabs/haneul/transactions";

type Options = Omit<Parameters<HaneulClient["getTransactionBlock"]>[0], "digest"> & {
    tx: Transaction;
};

type ExecuteResponse = { digest: string; rawEffects?: number[] };

type ExecuteCallback = ({
    bytes,
    signature,
}: {
    bytes: string;
    signature: string;
}) => Promise<ExecuteResponse>;

type ResponseCallback = (tx: HaneulTransactionBlockResponse) => void | Promise<void>;
type Executor = (options: Options, then: ResponseCallback) => void;

type ExecutorResult = {
    mutate: Executor;
    status: string;
    isIdle: boolean;
    isPending: boolean;
    isSuccess: boolean;
    isError: boolean;
    isPaused: boolean;
};

/**
 * Hook encapsulating running a transaction, waiting for its effects
 * and then doing something with them.
 */
export function useExecutor({ execute }: { execute?: ExecuteCallback } = {}): ExecutorResult {
    const client = useHaneulClient();
    const {
        mutate: signAndExecute,
        status,
        isIdle,
        isPending,
        isSuccess,
        isError,
        isPaused,
    } = useSignAndExecuteTransaction({ execute });

    const mutate: Executor = ({ tx, ...options }, then) => {
        signAndExecute(
            {
                // Fails with Transaction type version mismatch
                // @ts-ignore
                transaction: tx,
            },
            {
                onSuccess: ({ digest }) => {
                    client.waitForTransaction({ digest, ...options }).then(then);
                },

                onError: (error) => {
                    console.error("Failed to execute transaction", tx, error);
                },
            },
        );
    };

    return {
        mutate,
        status,
        isIdle,
        isPending,
        isSuccess,
        isError,
        isPaused,
    };
}
