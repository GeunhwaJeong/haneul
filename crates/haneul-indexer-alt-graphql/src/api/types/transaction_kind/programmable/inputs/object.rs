// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;

use haneul_types::base_types::ObjectID;
use haneul_types::base_types::SequenceNumber;
use haneul_types::digests::ObjectDigest;

use crate::api::scalars::haneul_address::HaneulAddress;
use crate::api::scalars::uint53::UInt53;
use crate::api::types::object::Object;
use crate::scope::Scope;

/// A Move object, either immutable, or owned mutable.
#[derive(SimpleObject)]
pub struct OwnedOrImmutable {
    pub object: Option<Object>,
}

/// A Move object that's shared.
#[derive(SimpleObject)]
pub struct SharedInput {
    /// The address of the shared object.
    pub address: Option<HaneulAddress>,

    /// The version that this object was shared at.
    pub initial_shared_version: Option<UInt53>,

    /// Controls whether the transaction block can reference the shared object as a mutable reference or by value.
    ///
    /// This has implications for scheduling: Transactions that just read shared objects at a certain version (mutable = false) can be executed concurrently, while transactions that write shared objects (mutable = true) must be executed serially with respect to each other.
    pub mutable: Option<bool>,
}

/// A Move object that can be received in this transaction.
#[derive(SimpleObject)]
pub struct Receiving {
    pub object: Option<Object>,
}

impl OwnedOrImmutable {
    pub fn from_object_ref(
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
        scope: Scope,
    ) -> Self {
        let object = Object::with_ref(&scope, object_id.into(), version, digest);
        Self {
            object: Some(object),
        }
    }
}

impl SharedInput {
    pub fn from_shared_object(
        object_id: ObjectID,
        initial_shared_version: SequenceNumber,
        mutable: bool,
    ) -> Self {
        Self {
            address: Some(object_id.into()),
            initial_shared_version: Some(initial_shared_version.value().into()),
            mutable: Some(mutable),
        }
    }
}

impl Receiving {
    pub fn from_object_ref(
        object_id: ObjectID,
        version: SequenceNumber,
        digest: ObjectDigest,
        scope: Scope,
    ) -> Self {
        let object = Object::with_ref(&scope, object_id.into(), version, digest);
        Self {
            object: Some(object),
        }
    }
}
