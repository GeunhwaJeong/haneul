// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use haneul_open_rpc_macros::open_rpc;
use haneul_types::bridge::BridgeSummary;

#[open_rpc(namespace = "haneulx", tag = "Bridge Read API")]
#[rpc(server, client, namespace = "haneulx")]
pub trait BridgeReadApi {
    /// Returns the latest BridgeSummary
    #[method(name = "getLatestBridge")]
    async fn get_latest_bridge(&self) -> RpcResult<BridgeSummary>;

    /// Returns the initial shared version of the bridge object, usually
    /// for the purpose of constructing an ObjectArg in a transaction.
    #[method(name = "getBridgeObjectInitialSharedVersion")]
    async fn get_bridge_object_initial_shared_version(&self) -> RpcResult<u64>;
}
