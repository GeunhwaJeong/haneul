// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use async_graphql::SimpleObject;

use crate::api::scalars::base64::Base64;
use crate::api::scalars::haneul_address::HaneulAddress;
use crate::api::types::transaction_kind::programmable::commands::TransactionArgument;

/// Upgrades a Move Package.
#[derive(SimpleObject, Clone)]
pub struct UpgradeCommand {
    /// Bytecode for the modules to be published, BCS serialized and Base64 encoded.
    pub modules: Option<Vec<Base64>>,

    /// IDs of the transitive dependencies of the package to be published.
    pub dependencies: Option<Vec<HaneulAddress>>,

    /// ID of the package being upgraded.
    pub current_package: Option<HaneulAddress>,

    /// The `UpgradeTicket` authorizing the upgrade.
    pub upgrade_ticket: Option<TransactionArgument>,
}
