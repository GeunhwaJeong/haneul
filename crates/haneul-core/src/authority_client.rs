// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use haneul_network::{api::ValidatorClient, tonic};
use haneul_types::base_types::AuthorityName;
use haneul_types::committee::CommitteeWithNetworkMetadata;
use haneul_types::crypto::NetworkPublicKey;
use haneul_types::error::{HaneulError, HaneulResult};
use haneul_types::haneul_system_state::HaneulSystemState;
use haneul_types::messages_checkpoint::{
    CheckpointRequest, CheckpointRequestV2, CheckpointResponse, CheckpointResponseV2,
};
use haneul_types::multiaddr::Multiaddr;
use haneullabs_network::config::Config;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tap::TapFallible;

use crate::authority_client::tonic::IntoRequest;
use haneul_network::tonic::metadata::KeyAndValueRef;
use haneul_network::tonic::transport::Channel;
use haneul_types::messages_grpc::{
    ObjectInfoRequest, ObjectInfoResponse, RawValidatorHealthRequest, RawWaitForEffectsRequest,
    SubmitTxRequest, SubmitTxResponse, SystemStateRequest, TransactionInfoRequest,
    TransactionInfoResponse, ValidatorHealthRequest, ValidatorHealthResponse,
    WaitForEffectsRequest, WaitForEffectsResponse,
};

#[async_trait]
pub trait AuthorityAPI {
    /// Submits a transaction to validators for sequencing and execution.
    async fn submit_transaction(
        &self,
        request: SubmitTxRequest,
        client_addr: Option<SocketAddr>,
    ) -> Result<SubmitTxResponse, HaneulError>;

    /// Waits for effects of a transaction that has been submitted to the network
    /// through the `submit_transaction` API.
    async fn wait_for_effects(
        &self,
        request: WaitForEffectsRequest,
        client_addr: Option<SocketAddr>,
    ) -> Result<WaitForEffectsResponse, HaneulError>;

    // TODO(fastpath): Add a soft bundle path for mfp which will return the list of consensus positions

    /// Handle Object information requests for this account.
    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, HaneulError>;

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, HaneulError>;

    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, HaneulError>;

    async fn handle_checkpoint_v2(
        &self,
        request: CheckpointRequestV2,
    ) -> Result<CheckpointResponseV2, HaneulError>;

    // This API is exclusively used by the benchmark code.
    // Hence it's OK to return a fixed system state type.
    async fn handle_system_state_object(
        &self,
        request: SystemStateRequest,
    ) -> Result<HaneulSystemState, HaneulError>;

    /// Get validator health metrics (for latency measurement)
    async fn validator_health(
        &self,
        request: ValidatorHealthRequest,
    ) -> Result<ValidatorHealthResponse, HaneulError>;

    /// Force the underlying transport to drop any cached connection and establish a fresh one on
    /// the next request. Used to recover from a connection that has silently gone dead (e.g. the
    /// peer validator restarted) and is no longer detected as broken by the transport layer.
    /// Implementations may rate-limit reconnections and ignore the request during the cooldown.
    /// Default is a no-op for client implementations without a reconnectable transport.
    fn reconnect(&self) {}
}

/// Builds a fresh lazy channel; used to re-establish a connection that has gone dead.
type ChannelBuilder = Arc<dyn Fn() -> HaneulResult<Channel> + Send + Sync>;

const RECONNECT_COOLDOWN: Duration = Duration::from_secs(2);

#[derive(Clone)]
pub struct NetworkAuthorityClient {
    /// The current (lazy) gRPC client, swappable so a dead connection can be replaced in place.
    client: Arc<ArcSwap<HaneulResult<ValidatorClient<Channel>>>>,
    /// When present, rebuilds a fresh channel on `reconnect()`. Absent for clients constructed
    /// directly from a `Channel` (e.g. tests), where reconnection is not possible.
    builder: Option<ChannelBuilder>,
    last_reconnect: Arc<Mutex<Option<Instant>>>,
}

impl NetworkAuthorityClient {
    pub async fn connect(
        address: &Multiaddr,
        tls_target: NetworkPublicKey,
    ) -> anyhow::Result<Self> {
        let tls_config = haneul_tls::create_rustls_client_config(
            tls_target,
            haneul_tls::HANEUL_VALIDATOR_SERVER_NAME.to_string(),
            None,
        );
        let channel = haneullabs_network::client::connect(address, tls_config)
            .await
            .map_err(|err| anyhow!(err.to_string()))?;
        Ok(Self::new(channel))
    }

    pub fn connect_lazy(address: &Multiaddr, tls_target: NetworkPublicKey) -> Self {
        let address = address.clone();
        Self::new_reconnectable(move || {
            let tls_config = haneul_tls::create_rustls_client_config(
                tls_target.clone(),
                haneul_tls::HANEUL_VALIDATOR_SERVER_NAME.to_string(),
                None,
            );
            haneullabs_network::client::connect_lazy(&address, tls_config)
                .map_err(|err| err.to_string().into())
        })
    }

    pub fn new(channel: Channel) -> Self {
        Self {
            client: Arc::new(ArcSwap::from_pointee(Ok(ValidatorClient::new(channel)))),
            builder: None,
            last_reconnect: Arc::new(Mutex::new(None)),
        }
    }

    /// Construct a client whose connection can be re-established via `reconnect()`. The builder is
    /// invoked once to create the initial lazy channel and again on each reconnect.
    pub(crate) fn new_reconnectable(
        builder: impl Fn() -> HaneulResult<Channel> + Send + Sync + 'static,
    ) -> Self {
        let initial = builder().map(ValidatorClient::new);
        Self {
            client: Arc::new(ArcSwap::from_pointee(initial)),
            builder: Some(Arc::new(builder)),
            last_reconnect: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn client(&self) -> HaneulResult<ValidatorClient<Channel>> {
        (**self.client.load()).clone()
    }

    pub fn get_client_for_testing(&self) -> HaneulResult<ValidatorClient<Channel>> {
        self.client()
    }
}

#[async_trait]
impl AuthorityAPI for NetworkAuthorityClient {
    /// Submits a transaction to the Haneul network for certification and execution.
    async fn submit_transaction(
        &self,
        request: SubmitTxRequest,
        client_addr: Option<SocketAddr>,
    ) -> Result<SubmitTxResponse, HaneulError> {
        let mut request = request.into_raw()?.into_request();
        insert_metadata(&mut request, client_addr);

        self.client()?
            .submit_transaction(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::<HaneulError>::into)?
            .try_into()
    }

    async fn wait_for_effects(
        &self,
        request: WaitForEffectsRequest,
        client_addr: Option<SocketAddr>,
    ) -> Result<WaitForEffectsResponse, HaneulError> {
        let raw_request: RawWaitForEffectsRequest = request.try_into()?;
        let mut request = raw_request.into_request();
        insert_metadata(&mut request, client_addr);

        self.client()?
            .wait_for_effects(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::<HaneulError>::into)?
            .try_into()
    }

    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, HaneulError> {
        self.client()?
            .object_info(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, HaneulError> {
        self.client()?
            .transaction_info(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Handle Object information requests for this account.
    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, HaneulError> {
        self.client()?
            .checkpoint(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    /// Handle Object information requests for this account.
    async fn handle_checkpoint_v2(
        &self,
        request: CheckpointRequestV2,
    ) -> Result<CheckpointResponseV2, HaneulError> {
        self.client()?
            .checkpoint_v2(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    async fn handle_system_state_object(
        &self,
        request: SystemStateRequest,
    ) -> Result<HaneulSystemState, HaneulError> {
        self.client()?
            .get_system_state_object(request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::into)
    }

    async fn validator_health(
        &self,
        request: ValidatorHealthRequest,
    ) -> Result<ValidatorHealthResponse, HaneulError> {
        let raw_request: RawValidatorHealthRequest = request.try_into()?;

        self.client()?
            .validator_health(raw_request)
            .await
            .map(tonic::Response::into_inner)
            .map_err(Into::<HaneulError>::into)?
            .try_into()
    }

    fn reconnect(&self) {
        let Some(builder) = &self.builder else {
            return;
        };
        let mut last_reconnect = self.last_reconnect.lock();
        if last_reconnect.is_some_and(|t| t.elapsed() < RECONNECT_COOLDOWN) {
            return;
        }
        let fresh = builder().map(ValidatorClient::new);
        self.client.store(Arc::new(fresh));
        *last_reconnect = Some(Instant::now());
    }
}

pub fn make_network_authority_clients_with_network_config(
    committee: &CommitteeWithNetworkMetadata,
    network_config: &Config,
) -> BTreeMap<AuthorityName, NetworkAuthorityClient> {
    let mut authority_clients = BTreeMap::new();
    for (name, (_state, network_metadata)) in committee.validators() {
        let address = network_metadata
            .network_address
            .clone()
            .rewrite_udp_to_tcp()
            .rewrite_http_to_https();
        let maybe_network_key = network_metadata.network_public_key.clone();
        let network_config = network_config.clone();
        let name = *name;
        // Build a fresh lazy channel on demand so a dead connection (e.g. after the peer
        // validator restarts) can be re-established via NetworkAuthorityClient::reconnect().
        let builder = move || -> HaneulResult<Channel> {
            let key = maybe_network_key
                .clone()
                .ok_or_else(|| HaneulError::from("network public key is not available"))?;
            let tls_config = haneul_tls::create_rustls_client_config(
                key,
                haneul_tls::HANEUL_VALIDATOR_SERVER_NAME.to_string(),
                None,
            );
            network_config
                .connect_lazy(&address, tls_config)
                .map_err(|e| e.to_string().into())
                .tap_err(|e| {
                    tracing::error!(
                        address = %address,
                        name = %name,
                        "unable to create authority client: {e}"
                    )
                })
        };
        let client = NetworkAuthorityClient::new_reconnectable(builder);
        authority_clients.insert(name, client);
    }
    authority_clients
}

pub fn make_authority_clients_with_timeout_config(
    committee: &CommitteeWithNetworkMetadata,
    connect_timeout: Duration,
    request_timeout: Duration,
) -> BTreeMap<AuthorityName, NetworkAuthorityClient> {
    let mut network_config = haneullabs_network::config::Config::new();
    network_config.connect_timeout = Some(connect_timeout);
    network_config.request_timeout = Some(request_timeout);
    network_config.http2_keepalive_interval = Some(connect_timeout);
    network_config.http2_keepalive_timeout = Some(connect_timeout);
    make_network_authority_clients_with_network_config(committee, &network_config)
}

fn insert_metadata<T>(request: &mut tonic::Request<T>, client_addr: Option<SocketAddr>) {
    if let Some(client_addr) = client_addr {
        let mut metadata = tonic::metadata::MetadataMap::new();
        metadata.insert("x-forwarded-for", client_addr.to_string().parse().unwrap());
        metadata
            .iter()
            .for_each(|key_and_value| match key_and_value {
                KeyAndValueRef::Ascii(key, value) => {
                    request.metadata_mut().insert(key, value.clone());
                }
                KeyAndValueRef::Binary(key, value) => {
                    request.metadata_mut().insert_bin(key, value.clone());
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[tokio::test]
    async fn test_reconnect_cooldown() {
        let build_count = Arc::new(AtomicUsize::new(0));
        let build_count_clone = build_count.clone();
        let client = NetworkAuthorityClient::new_reconnectable(move || {
            build_count_clone.fetch_add(1, Ordering::SeqCst);
            Ok(tonic::transport::Endpoint::from_static("http://127.0.0.1:1").connect_lazy())
        });
        assert_eq!(build_count.load(Ordering::SeqCst), 1);

        client.reconnect();
        assert_eq!(build_count.load(Ordering::SeqCst), 2);

        client.reconnect();
        assert_eq!(build_count.load(Ordering::SeqCst), 2);

        tokio::time::sleep(RECONNECT_COOLDOWN + Duration::from_millis(100)).await;
        client.reconnect();
        assert_eq!(build_count.load(Ordering::SeqCst), 3);
    }
}
