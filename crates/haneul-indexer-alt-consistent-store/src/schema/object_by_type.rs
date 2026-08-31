// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use bincode::Decode;
use bincode::Encode;
use haneul_indexer_alt_framework::types::base_types::ObjectID;
use haneul_indexer_alt_framework::types::object::Object;
use move_core_types::language_storage::StructTag;

/// Key for the index that supports fetching objects by their type.
#[derive(Encode, Decode, PartialEq, Eq)]
pub(crate) struct Key {
    /// The object's type (only MoveObjects are indexed)
    #[bincode(with_serde)]
    pub(crate) type_: StructTag,

    /// The ID of the object.
    #[bincode(with_serde)]
    pub(crate) object_id: ObjectID,
}

impl Key {
    pub(crate) fn from_object(obj: &Object) -> Option<Key> {
        Some(Key {
            type_: obj.type_()?.clone().into(),
            object_id: obj.id(),
        })
    }
}

/// Options for creating this index's column family in RocksDB.
pub(crate) fn options(base_options: &rocksdb::Options) -> rocksdb::Options {
    base_options.clone()
}
