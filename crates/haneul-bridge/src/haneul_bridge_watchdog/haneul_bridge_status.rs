// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! The HaneulBridgeStatus observable monitors whether the Haneul Bridge is paused.

use crate::haneul_bridge_watchdog::Observable;
use crate::haneul_client::HaneulBridgeClient;
use async_trait::async_trait;
use prometheus::IntGauge;
use std::sync::Arc;

use tokio::time::Duration;
use tracing::{error, info};

pub struct HaneulBridgeStatus {
    haneul_client: Arc<HaneulBridgeClient>,
    metric: IntGauge,
}

impl HaneulBridgeStatus {
    pub fn new(haneul_client: Arc<HaneulBridgeClient>, metric: IntGauge) -> Self {
        Self {
            haneul_client,
            metric,
        }
    }
}

#[async_trait]
impl Observable for HaneulBridgeStatus {
    fn name(&self) -> &str {
        "HaneulBridgeStatus"
    }

    async fn observe_and_report(&self) {
        let status = self.haneul_client.is_bridge_paused().await;
        match status {
            Ok(status) => {
                self.metric.set(status as i64);
                info!("Haneul Bridge Status: {:?}", status);
            }
            Err(e) => {
                error!("Error getting haneul bridge status: {:?}", e);
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
}
