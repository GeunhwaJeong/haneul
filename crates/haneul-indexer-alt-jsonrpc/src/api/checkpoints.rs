// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use haneul_json_rpc_types::Checkpoint;
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::haneul_serde::BigInt;
use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use crate::api::rpc_module::RpcModule;
use crate::context::Context;
use crate::error::InternalContext;
use crate::error::RpcError;
use crate::error::invalid_params;

#[open_rpc(namespace = "haneul", tag = "Checkpoints API")]
#[rpc(server, namespace = "haneul")]
trait CheckpointsApi {
    /// Return a checkpoint by its sequence number
    #[method(name = "getCheckpoint")]
    async fn get_checkpoint(
        &self,
        /// Checkpoint sequence number.
        seq: BigInt<u64>,
    ) -> RpcResult<Checkpoint>;
}

pub(crate) struct Checkpoints(pub Context);

#[derive(thiserror::Error, Debug, Clone)]
enum Error {
    #[error("Checkpoint {0} not found")]
    NotFound(u64),
}

#[async_trait::async_trait]
impl CheckpointsApiServer for Checkpoints {
    async fn get_checkpoint(&self, seq: BigInt<u64>) -> RpcResult<Checkpoint> {
        let Self(ctx) = self;
        Ok(response(ctx, *seq).await.with_internal_context(|| {
            format!("Failed to fetch checkpoint at sequence number {seq:?}")
        })?)
    }
}

impl RpcModule for Checkpoints {
    fn schema(&self) -> Module {
        CheckpointsApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}

/// Load a checkpoint and prepare it for presentation as a JSON-RPC response.
async fn response(ctx: &Context, seq: u64) -> Result<Checkpoint, RpcError<Error>> {
    let (summary, contents, signature) = ctx
        .kv_loader()
        .load_one_checkpoint(seq)
        .await
        .context("Failed to load checkpoint")?
        .ok_or_else(|| invalid_params(Error::NotFound(seq)))?;

    Ok(Checkpoint::from((summary, contents, signature.signature)))
}
