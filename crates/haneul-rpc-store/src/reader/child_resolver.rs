// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! [`RuntimeObjectResolver`] adapter.
//!
//! `RuntimeObjectResolver` is one of the supertrait bounds on
//! [`haneul_types::storage::RpcStateReader`]. Its methods feed the
//! Move runtime's dynamic-field / receive-object paths. **This
//! adapter is read-only and does not serve Move execution**: it
//! returns `Ok(None)` from both methods.
//!
//! Callers that need execution-time child-object resolution
//! (re-running a transaction, simulating dry-runs) should keep
//! using the validator perpetual store — this adapter is meant
//! for the read-only RPC surface where child-object lookups never
//! arise.

use haneul_consistent_store::reader::Reader;
use haneul_types::base_types::EpochId;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::SequenceNumber;
use haneul_types::error::HaneulResult;
use haneul_types::object::Object;
use haneul_types::storage::{BackingPackageStore, PackageObject, RuntimeObjectResolver};

use crate::reader::RpcStoreReader;

impl<R: Reader + Send + Sync> BackingPackageStore for RpcStoreReader<R> {
    fn get_package_object(&self, _package_id: &ObjectID) -> HaneulResult<Option<PackageObject>> {
        Ok(None)
    }
}

impl<R: Reader + Send + Sync> RuntimeObjectResolver for RpcStoreReader<R> {
    fn read_child_object(
        &self,
        _parent: &ObjectID,
        _child: &ObjectID,
        _child_version_upper_bound: SequenceNumber,
    ) -> HaneulResult<Option<Object>> {
        Ok(None)
    }

    fn get_object_received_at_version(
        &self,
        _owner: &ObjectID,
        _receiving_object_id: &ObjectID,
        _receive_object_at_version: SequenceNumber,
        _epoch_id: EpochId,
    ) -> HaneulResult<Option<Object>> {
        Ok(None)
    }
}
