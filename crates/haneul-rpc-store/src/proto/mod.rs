// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Generated protobuf types backing `haneul-rpc-store` values.
//!
//! Regenerate with `cargo +nightly -Zscript codegen.rs` from the
//! crate root; see `codegen.rs` for details.

#[allow(clippy::all)]
pub mod haneul {
    pub mod rpc_store {
        pub mod v1alpha {
            include!("generated/haneul.rpc_store.v1alpha.rs");
            include!("generated/haneul.rpc_store.v1alpha.accessors.rs");
        }
    }
}

pub use haneul::rpc_store::v1alpha::BalanceDelta;
pub use haneul::rpc_store::v1alpha::BitmapBlob;
pub use haneul::rpc_store::v1alpha::ObjectVersionInfo;
pub use haneul::rpc_store::v1alpha::PackageVersionInfo;
pub use haneul::rpc_store::v1alpha::PruningWatermarks;
pub use haneul::rpc_store::v1alpha::StoredCheckpointContents;
pub use haneul::rpc_store::v1alpha::StoredCheckpointSummary;
pub use haneul::rpc_store::v1alpha::StoredEffects;
pub use haneul::rpc_store::v1alpha::StoredEpoch;
pub use haneul::rpc_store::v1alpha::StoredEvents;
pub use haneul::rpc_store::v1alpha::StoredObject;
pub use haneul::rpc_store::v1alpha::StoredObjectTombstone;
pub use haneul::rpc_store::v1alpha::StoredObjectTombstoneKind;
pub use haneul::rpc_store::v1alpha::StoredTransaction;
pub use haneul::rpc_store::v1alpha::TxMetadata;
pub use haneul::rpc_store::v1alpha::stored_object;
