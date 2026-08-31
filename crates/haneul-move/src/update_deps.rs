// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

use haneul_sdk::wallet_context::WalletContext;
use move_cli::base::{reroot_path, update_deps};
use move_package_alt_compilation::build_config::BuildConfig;

use std::path::Path;

use haneul_package_alt::{HaneulFlavor, find_environment};

#[derive(Parser)]
#[group(id = "haneul-move-update-deps")]
pub struct UpdateDeps {
    #[clap(flatten)]
    pub update_deps: update_deps::UpdateDeps,
}

impl UpdateDeps {
    pub async fn execute(
        self,
        path: Option<&Path>,
        build_config: BuildConfig,
        wallet: &WalletContext,
        flavor: HaneulFlavor,
    ) -> anyhow::Result<()> {
        let path = reroot_path(path)?;
        let environment =
            find_environment(&path, build_config.environment.clone(), wallet, false).await?;
        self.update_deps
            .execute(Some(&path), &build_config, environment, flavor)
            .await
    }
}
