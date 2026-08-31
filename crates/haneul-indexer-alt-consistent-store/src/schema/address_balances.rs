// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use bincode::Decode;
use bincode::Encode;
use haneul_indexer_alt_framework::types::base_types::HaneulAddress;
use move_core_types::language_storage::TypeTag;

#[derive(Encode, Decode, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) struct Key {
    #[bincode(with_serde)]
    pub(crate) owner: HaneulAddress,

    /// The inner type of some balance `Balance<T>`, e.g. for `0x2::balance::Balance<0x2::haneul::HANEUL>`
    /// this would be `0x2::haneul::HANEUL`.
    #[bincode(with_serde)]
    pub(crate) type_: TypeTag,
}

/// Options for creating this index's column family in RocksDB.
pub(crate) fn options(base_options: &rocksdb::Options) -> rocksdb::Options {
    base_options.clone()
}
