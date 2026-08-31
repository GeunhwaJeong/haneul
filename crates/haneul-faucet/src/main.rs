// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use haneul_config::haneul_config_dir;
use haneul_faucet::{AppState, create_wallet_context, start_faucet};
use haneul_faucet::{FaucetConfig, LocalFaucet};
use haneul_futures::service::Error as ServiceError;
use std::env;
use std::sync::Arc;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config: FaucetConfig = FaucetConfig::parse();
    let FaucetConfig {
        wallet_client_timeout_secs,
        ..
    } = config;

    let context = create_wallet_context(wallet_client_timeout_secs, haneul_config_dir()?)?;

    let app_state = Arc::new(AppState {
        faucet: LocalFaucet::new(context, config.clone()).await.unwrap(),
        config,
    });

    match start_faucet(app_state).await?.main().await {
        Ok(()) | Err(ServiceError::Terminated) => Ok(()),
        Err(ServiceError::Aborted) => Err(anyhow::anyhow!("Faucet aborted during shutdown")),
        Err(ServiceError::Task(e)) => Err(e),
    }
}
