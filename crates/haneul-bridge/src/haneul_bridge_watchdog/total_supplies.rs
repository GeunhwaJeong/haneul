// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! The HaneulBridgeStatus observable monitors whether the Haneul Bridge is paused.

use crate::haneul_bridge_watchdog::Observable;
use async_trait::async_trait;
use haneul_rpc::proto::haneul::rpc::v2::GetCoinInfoRequest;
use prometheus::IntGaugeVec;
use std::collections::BTreeMap;

use tokio::time::Duration;
use tracing::{error, info};

pub struct TotalSupplies {
    haneul_client: haneul_rpc::Client,
    coins: BTreeMap<String, String>,
    metric: IntGaugeVec,
}

impl TotalSupplies {
    pub fn new(
        haneul_client: haneul_rpc::Client,
        coins: BTreeMap<String, String>,
        metric: IntGaugeVec,
    ) -> Self {
        Self {
            haneul_client,
            coins,
            metric,
        }
    }
}

#[async_trait]
impl Observable for TotalSupplies {
    fn name(&self) -> &str {
        "TotalSupplies"
    }

    async fn observe_and_report(&self) {
        for (coin_name, coin_type) in &self.coins {
            let resp = self
                .haneul_client
                .clone()
                .state_client()
                .get_coin_info(GetCoinInfoRequest::default().with_coin_type(coin_type))
                .await;
            match resp {
                Ok(resp) => {
                    let supply = resp.into_inner().treasury().total_supply();
                    self.metric
                        .with_label_values(&[coin_name])
                        .set(supply as i64);
                    info!("Total supply for {coin_type}: {}", supply);
                }
                Err(e) => {
                    error!("Error getting total supply for coin {coin_type}: {:?}", e);
                }
            }
        }
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10)
    }
}
