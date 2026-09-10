// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::Context;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2 as proto;
use haneul_types::base_types::ObjectID;
use haneul_types::object::Object;
use prost_types::FieldMask;

use crate::error::Error;
use crate::ledger_grpc_reader::ChunkedLoader;
use crate::ledger_grpc_reader::LedgerGrpcReader;

/// Key for fetching the contents a particular version of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VersionedObjectKey(pub ObjectID, pub u64);

#[async_trait::async_trait]
impl ChunkedLoader<VersionedObjectKey> for LedgerGrpcReader {
    type Value = Object;
    type Error = Error;

    fn chunk_size(&self) -> usize {
        self.max_batch_get_objects()
    }

    async fn load_chunk(
        &self,
        keys: &[VersionedObjectKey],
    ) -> Result<HashMap<VersionedObjectKey, Object>, Error> {
        let requests = keys
            .iter()
            .map(|key| {
                let mut req = proto::GetObjectRequest::new(&key.0.into());
                req.version = Some(key.1);
                req
            })
            .collect();

        let mut request = proto::BatchGetObjectsRequest::default();
        request.requests = requests;
        request.read_mask = Some(FieldMask::from_paths(["bcs"]));

        let batch_response = self.batch_get_objects(request).await?;

        let mut results = HashMap::new();
        for obj_result in batch_response.objects {
            if let Some(proto::get_object_result::Result::Object(object)) = obj_result.result {
                let obj: Object = object
                    .bcs
                    .as_ref()
                    .context("Missing bcs in object")?
                    .deserialize()
                    .context("Failed to deserialize object")?;
                results.insert(VersionedObjectKey(obj.id(), obj.version().into()), obj);
            }
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use async_graphql::dataloader::Loader;
    use haneul_sdk_types::Address;
    use haneul_types::base_types::ObjectID;

    use super::*;
    use crate::ledger_grpc_reader::test_support::mock_reader;

    #[tokio::test]
    async fn load_chunks_oversized_batches() {
        let (reader, mock, server) = mock_reader().await;
        let limit = reader.max_batch_get_objects();

        let keys: Vec<VersionedObjectKey> = (0..limit + 50)
            .map(|i| VersionedObjectKey(ObjectID::random(), i as u64))
            .collect();

        let result = reader.load(&keys).await.expect("load should succeed");
        assert!(result.is_empty());

        let batches = mock.object_batches();
        assert_eq!(batches.len(), 2);
        assert!(batches.iter().all(|batch| batch.len() <= limit));

        let mut requested: Vec<(String, Option<u64>)> = batches
            .into_iter()
            .flatten()
            .map(|req| (req.object_id.unwrap_or_default(), req.version))
            .collect();
        requested.sort();
        let mut expected: Vec<(String, Option<u64>)> = keys
            .iter()
            .map(|key| {
                let address: Address = key.0.into();
                (address.to_string(), Some(key.1))
            })
            .collect();
        expected.sort();
        assert_eq!(requested, expected);

        server.abort();
    }
}
