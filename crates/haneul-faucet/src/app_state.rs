// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::FaucetConfig;
use crate::LocalFaucet;
use std::sync::Arc;

pub struct AppState<F = Arc<LocalFaucet>> {
    pub faucet: F,
    pub config: FaucetConfig,
}

impl<F> AppState<F> {
    pub fn new(faucet: F, config: FaucetConfig) -> Self {
        Self { faucet, config }
    }
}
