// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use futures::StreamExt;
use futures::stream::BoxStream;
use haneul_rpc_api::RpcError;
use haneul_types::object::Object;
use haneul_types::storage::ObjectKey;

use crate::bigtable_client::BigTableClient;
use crate::request_cache::CacheFetcher;

pub(crate) type ObjectMap = Arc<HashMap<ObjectKey, Object>>;

pub(crate) struct BigTableObjectFetcher {
    client: BigTableClient,
}

impl BigTableObjectFetcher {
    pub(crate) fn new(client: BigTableClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl CacheFetcher<ObjectKey, Object> for BigTableObjectFetcher {
    async fn fetch(
        &self,
        keys: Vec<ObjectKey>,
    ) -> Result<BoxStream<'static, Result<(ObjectKey, Object), RpcError>>, RpcError> {
        let stream = self.client.get_objects_stream(keys).await?;
        Ok(stream
            .map(|row| {
                row.map(|obj| (ObjectKey(obj.id(), obj.version()), obj))
                    .map_err(RpcError::from)
            })
            .boxed())
    }
}
