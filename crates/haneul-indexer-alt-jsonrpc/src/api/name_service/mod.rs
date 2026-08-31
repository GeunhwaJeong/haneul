// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_json_rpc_types::Page as PageResponse;
use haneul_open_rpc::Module;
use haneul_open_rpc_macros::open_rpc;
use haneul_types::base_types::HaneulAddress;
use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use crate::api::name_service::error::Error;
use crate::api::rpc_module::RpcModule;
use crate::context::Context;
use crate::error::InternalContext as _;

mod error;
mod response;

#[open_rpc(namespace = "haneulx", tag = "Name Service API")]
#[rpc(server, namespace = "haneulx")]
trait NameServiceApi {
    /// Resolve a HaneulNS name to its address
    #[method(name = "resolveNameServiceAddress")]
    async fn resolve_name_service_address(
        &self,
        /// The name to resolve
        name: String,
    ) -> RpcResult<Option<HaneulAddress>>;

    /// Find the HaneulNS name that points to this address.
    ///
    /// Although this method's response is paginated, it will only ever return at most one name.
    #[method(name = "resolveNameServiceNames")]
    async fn resolve_name_service_names(
        &self,
        /// The address to resolve
        address: HaneulAddress,
        /// Unused pagination cursor
        cursor: Option<String>,
        /// Unused pagination limit
        limit: Option<usize>,
    ) -> RpcResult<PageResponse<String, String>>;
}

pub(crate) struct NameService(pub Context);

#[async_trait::async_trait]
impl NameServiceApiServer for NameService {
    async fn resolve_name_service_address(&self, name: String) -> RpcResult<Option<HaneulAddress>> {
        let Self(ctx) = self;
        Ok(response::resolved_address(ctx, &name)
            .await
            .with_internal_context(|| format!("Resolving HaneulNS name {name:?}"))?)
    }

    async fn resolve_name_service_names(
        &self,
        address: HaneulAddress,
        _cursor: Option<String>,
        _limit: Option<usize>,
    ) -> RpcResult<PageResponse<String, String>> {
        let Self(ctx) = self;

        let mut page = PageResponse::empty();
        if let Some(name) = response::resolved_name(ctx, address)
            .await
            .with_internal_context(|| format!("Resolving address {address}"))?
        {
            page.data.push(name);
        }

        Ok(page)
    }
}

impl RpcModule for NameService {
    fn schema(&self) -> Module {
        NameServiceApiOpenRpc::module_doc()
    }

    fn into_impl(self) -> jsonrpsee::RpcModule<Self> {
        self.into_rpc()
    }
}
