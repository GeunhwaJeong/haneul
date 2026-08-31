// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_package_alt::HaneulFlavor;
use move_package_alt::{
    cache_package,
    schema::{Environment, ManifestDependencyInfo},
};
use serde::Deserialize;

/// Download a package and return information about it. Note that local packages must use the
/// absolute path.
#[derive(Parser)]
pub struct CachePackage {
    /// The environment name to use for resolution
    environment_name: String,

    /// The chain ID to use for resolution
    environment_id: String,

    /// A string containing the dependency as it would appear in Move.toml (e.g. `{ git = "...",
    /// rev = "...", subdir = "..." }`)
    dependency: String,
}

#[derive(Deserialize)]
struct DepSpec {
    dep: ManifestDependencyInfo,
}

impl CachePackage {
    pub async fn execute(&self, flavor: HaneulFlavor) -> anyhow::Result<()> {
        let str = format!("dep = {}", self.dependency);
        let dep: DepSpec = toml::from_str(&str)?;
        let env = Environment {
            name: self.environment_name.clone(),
            id: self.environment_id.clone(),
        };
        let info = cache_package(&env, &dep.dep, flavor).await?;
        println!("{}", serde_json::to_string(&info)?);

        Ok(())
    }
}
