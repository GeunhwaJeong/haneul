// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use haneul_core::{
    authority_aggregator::AuthorityAggregator,
    authority_client::NetworkAuthorityClient,
    epoch::committee_store::CommitteeStore,
    safe_client::SafeClientMetricsBase,
    transaction_driver::{AuthorityAggregatorUpdatable, ReconfigObserver},
};
use haneul_rpc_api::Client;
use std::sync::Arc;
use tracing::{debug, error, trace};

/// A ReconfigObserver that polls FullNode periodically
/// to get new epoch information.
/// Caveat: it does not guarantee to insert every committee
/// into committee store. This is fine in scenarios such
/// as stress, but may not be suitable in some other cases.
#[derive(Clone)]
pub struct FullNodeReconfigObserver {
    pub fullnode_client: Client,
    committee_store: Arc<CommitteeStore>,
    safe_client_metrics_base: SafeClientMetricsBase,
}

impl FullNodeReconfigObserver {
    pub async fn new(
        fullnode_rpc_url: &str,
        committee_store: Arc<CommitteeStore>,
        safe_client_metrics_base: SafeClientMetricsBase,
    ) -> Self {
        Self {
            fullnode_client: Client::new(fullnode_rpc_url).unwrap_or_else(|e| {
                panic!(
                    "Can't create HaneulClient with rpc url {fullnode_rpc_url}: {:?}",
                    e
                )
            }),
            committee_store,
            safe_client_metrics_base,
        }
    }
}

#[async_trait]
impl ReconfigObserver<NetworkAuthorityClient> for FullNodeReconfigObserver {
    fn clone_boxed(&self) -> Box<dyn ReconfigObserver<NetworkAuthorityClient> + Send + Sync> {
        Box::new(self.clone())
    }

    async fn run(&mut self, driver: Arc<dyn AuthorityAggregatorUpdatable<NetworkAuthorityClient>>) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            match self.fullnode_client.get_system_state_summary(None).await {
                Ok(haneul_system_state) => {
                    let epoch_id = haneul_system_state.epoch;
                    if epoch_id > driver.epoch() {
                        debug!(epoch_id, "Got HaneulSystemState in newer epoch");
                        let new_committee =
                            haneul_system_state.get_haneul_committee_for_benchmarking();
                        let _ = self
                            .committee_store
                            .insert_new_committee(new_committee.committee());
                        let auth_agg = AuthorityAggregator::new_from_committee(
                            new_committee,
                            Arc::new(
                                haneul_system_state.get_committee_authority_names_to_hostnames(),
                            ),
                            haneul_system_state.reference_gas_price,
                            &self.committee_store,
                            self.safe_client_metrics_base.clone(),
                        );
                        driver.update_authority_aggregator(Arc::new(auth_agg));
                    } else {
                        trace!(
                            epoch_id,
                            "Ignored SystemState from a previous or current epoch",
                        );
                    }
                }
                Err(err) => error!("Can't get HaneulSystemState from Full Node: {:?}", err,),
            }
        }
    }
}
