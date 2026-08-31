// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_package_alt::HaneulFlavor;
use move_cli::base::migrate;
use move_package_alt_compilation::build_config::BuildConfig;
use std::path::Path;

#[derive(Parser)]
#[group(id = "haneul-move-migrate")]
pub struct Migrate {
    #[clap(flatten)]
    pub migrate: migrate::Migrate,
}

impl Migrate {
    pub async fn execute(
        self,
        path: Option<&Path>,
        config: BuildConfig,
        flavor: HaneulFlavor,
    ) -> anyhow::Result<()> {
        self.migrate.execute(path, config, flavor).await
    }
}
