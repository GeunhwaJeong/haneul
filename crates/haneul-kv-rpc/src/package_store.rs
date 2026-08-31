// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use haneul_kvstore::{BigTableClient, KeyValueStoreReader};
use haneul_package_resolver::Package;
use haneul_package_resolver::PackageStore;
use haneul_package_resolver::error::Error;
use haneul_types::base_types::ObjectID;
use move_core_types::account_address::AccountAddress;

const STORE: &str = "BigTable";

pub struct BigTablePackageStore {
    client: BigTableClient,
}

impl BigTablePackageStore {
    pub fn new(client: BigTableClient) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl PackageStore for BigTablePackageStore {
    async fn fetch(&self, id: AccountAddress) -> haneul_package_resolver::Result<Arc<Package>> {
        let mut client = self.client.clone();
        let object = client
            .get_latest_object(&ObjectID::from(id))
            .await
            .map_err(|e| Error::Store {
                store: STORE,
                error: e.to_string(),
            })?
            .ok_or(Error::PackageNotFound(id))?;
        let package = object
            .data
            .try_as_package()
            .ok_or(Error::NotAPackage(object.id().into()))?;
        Ok(Arc::new(Package::read_from_package(package)?))
    }
}
