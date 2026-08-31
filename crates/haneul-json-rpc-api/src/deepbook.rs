// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use haneul_open_rpc_macros::open_rpc;

#[open_rpc(namespace = "haneulx", tag = "DeepBook Read API")]
#[rpc(server, client, namespace = "haneulx")]
pub trait DeepBookApi {
    #[method(name = "ping")]
    async fn ping(&self) -> RpcResult<String>;
}
