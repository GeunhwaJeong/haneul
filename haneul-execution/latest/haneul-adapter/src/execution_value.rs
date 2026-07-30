// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use haneul_types::storage::{BackingPackageStore, RuntimeObjectResolver, Storage};

pub trait HaneulResolver: BackingPackageStore {
    fn as_backing_package_store(&self) -> &dyn BackingPackageStore;
}

impl<T> HaneulResolver for T
where
    T: BackingPackageStore,
{
    fn as_backing_package_store(&self) -> &dyn BackingPackageStore {
        self
    }
}

/// Interface with the store necessary to execute a programmable transaction
pub trait ExecutionState: Storage + RuntimeObjectResolver + HaneulResolver {
    fn as_child_resolver(&self) -> &dyn RuntimeObjectResolver;
}

impl<T> ExecutionState for T
where
    T: Storage + RuntimeObjectResolver,
    T: HaneulResolver,
{
    fn as_child_resolver(&self) -> &dyn RuntimeObjectResolver {
        self
    }
}
