// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_package_alt::HaneulFlavor;
use move_cli::base::coverage;
use move_package_alt_compilation::build_config::BuildConfig;
use std::path::Path;

#[derive(Parser)]
#[group(id = "haneul-move-coverage")]
pub struct Coverage {
    #[clap(flatten)]
    pub coverage: coverage::Coverage,
}

impl Coverage {
    pub async fn execute(
        self,
        path: Option<&Path>,
        build_config: BuildConfig,
        flavor: HaneulFlavor,
    ) -> anyhow::Result<()> {
        self.coverage.execute(path, build_config, flavor).await?;
        Ok(())
    }
}
