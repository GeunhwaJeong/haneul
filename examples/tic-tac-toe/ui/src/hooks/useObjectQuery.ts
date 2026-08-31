// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClientContext, useHaneulClientQuery, UseHaneulClientQueryOptions } from "@haneullabs/dapp-kit";
import { GetObjectParams, HaneulObjectResponse } from "@haneullabs/haneul/client";
import { useQueryClient, UseQueryResult } from "@tanstack/react-query";

export type UseObjectQueryOptions = UseHaneulClientQueryOptions<"getObject", HaneulObjectResponse>;
export type UseObjectQueryResponse = UseQueryResult<HaneulObjectResponse, Error>;
export type InvalidateUseObjectQuery = () => void;

/**
 * Fetches an object, returning the response from RPC and a callback
 * to invalidate it.
 */
export function useObjectQuery(
    params: GetObjectParams,
    options?: UseObjectQueryOptions,
): [UseObjectQueryResponse, InvalidateUseObjectQuery] {
    const ctx = useHaneulClientContext();
    const client = useQueryClient();
    const response = useHaneulClientQuery("getObject", params, options);

    const invalidate = async () => {
        await client.invalidateQueries({
            queryKey: [ctx.network, "getObject", params],
        });
    };

    return [response, invalidate];
}
