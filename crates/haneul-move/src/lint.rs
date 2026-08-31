// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_package_alt::HaneulFlavor;
use move_cli::base::lint;
use move_compiler::editions::Flavor;
use move_package_alt_compilation::build_config::BuildConfig;
use std::path::Path;

#[derive(Parser)]
#[group(id = "haneul-move-lint")]
pub struct Lint {
    #[clap(flatten)]
    pub lint: lint::Lint,
}

impl Lint {
    pub async fn execute(
        self,
        path: Option<&Path>,
        mut build_config: BuildConfig,
        flavor: HaneulFlavor,
    ) -> anyhow::Result<()> {
        // Force the Haneul compiler flavor (as `build` and `test` do) so that the Haneul-specific
        // linters are registered. Without this, `haneul move lint` runs only the generic Move
        // linters and silently skips the Haneul object-model lints.
        if build_config.default_flavor.is_none() {
            build_config.default_flavor = Some(Flavor::Haneul);
        }
        self.lint.execute(path, build_config, flavor).await
    }
}
