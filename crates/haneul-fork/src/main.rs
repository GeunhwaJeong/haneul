// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;

use haneul_fork::cli::Cli;

bin_version::bin_version!();

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    Cli::parse_with_version(VERSION).execute(VERSION).await
}
