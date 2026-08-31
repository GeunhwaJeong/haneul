// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::string::ToString;
use std::sync::Arc;

use axum::routing::post;
use axum::{Extension, Router};
use lru::LruCache;
use move_core_types::language_storage::TypeTag;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tracing::info;

use haneul_rpc::client::Client;
use haneul_rpc::field::FieldMaskUtil;
use haneul_rpc::proto::haneul::rpc::v2::{GetCoinInfoRequest, GetEpochRequest};
use haneul_sdk_types::{StructTag, TypeTag as SDKTypeTag};
use haneul_types::digests::ChainIdentifier;
use prost_types::FieldMask;

use crate::errors::Error;
use crate::errors::Error::MissingMetadata;

pub use crate::errors::Error as RosettaError;
use crate::state::{CheckpointBlockProvider, OnlineServerContext};
use crate::types::{Currency, CurrencyMetadata, HaneulEnv};

/// This lib implements the Mesh online and offline server defined by the [Mesh API Spec](https://docs.cdp.coinbase.com/mesh/mesh-api-spec/api-reference)
mod account;
mod block;
mod construction;
pub mod errors;
mod network;
pub mod operations;
mod state;
pub mod types;

pub(crate) async fn get_current_epoch(client: &mut Client) -> Result<u64, Error> {
    let request = GetEpochRequest::latest().with_read_mask(FieldMask::from_paths(["epoch"]));
    Ok(client
        .ledger_client()
        .get_epoch(request)
        .await?
        .into_inner()
        .epoch()
        .epoch())
}

pub static HANEUL: Lazy<Currency> = Lazy::new(|| Currency {
    symbol: "HANEUL".to_string(),
    decimals: 9,
    metadata: CurrencyMetadata {
        coin_type: SDKTypeTag::from(StructTag::haneul()).to_string(),
    },
});

pub struct RosettaOnlineServer {
    env: HaneulEnv,
    context: OnlineServerContext,
}

impl RosettaOnlineServer {
    pub fn new(env: HaneulEnv, client: Client, chain_id: ChainIdentifier) -> Self {
        let coin_cache = CoinMetadataCache::new(client.clone(), NonZeroUsize::new(1000).unwrap());
        let blocks = Arc::new(CheckpointBlockProvider::new(
            client.clone(),
            coin_cache.clone(),
        ));

        Self {
            env,
            context: OnlineServerContext::new(client, blocks, coin_cache, chain_id),
        }
    }

    pub async fn serve(self, addr: SocketAddr) {
        // Online endpoints
        let app = Router::new()
            .route("/account/balance", post(account::balance))
            .route("/account/coins", post(account::coins))
            .route("/block", post(block::block))
            .route("/block/transaction", post(block::transaction))
            .route("/construction/submit", post(construction::submit))
            .route("/construction/metadata", post(construction::metadata))
            .route("/network/status", post(network::status))
            .route("/network/list", post(network::list))
            .route("/network/options", post(network::options))
            .layer(Extension(self.env))
            .with_state(self.context);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        info!(
            "Haneul Rosetta online server listening on {}",
            listener.local_addr().unwrap()
        );
        axum::serve(listener, app).await.unwrap();
    }
}

pub struct RosettaOfflineServer {
    env: HaneulEnv,
}

impl RosettaOfflineServer {
    pub fn new(env: HaneulEnv) -> Self {
        Self { env }
    }

    pub async fn serve(self, addr: SocketAddr) {
        // Online endpoints
        let app = Router::new()
            .route("/construction/derive", post(construction::derive))
            .route("/construction/payloads", post(construction::payloads))
            .route("/construction/combine", post(construction::combine))
            .route("/construction/preprocess", post(construction::preprocess))
            .route("/construction/hash", post(construction::hash))
            .route("/construction/parse", post(construction::parse))
            .route("/network/list", post(network::list))
            .route("/network/options", post(network::options))
            .layer(Extension(self.env));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        info!(
            "Haneul Rosetta offline server listening on {}",
            listener.local_addr().unwrap()
        );
        axum::serve(listener, app).await.unwrap();
    }
}

#[derive(Clone)]
pub struct CoinMetadataCache {
    client: Client,
    metadata: Arc<Mutex<LruCache<TypeTag, Currency>>>,
}

impl CoinMetadataCache {
    pub fn new(client: Client, size: NonZeroUsize) -> Self {
        Self {
            client,
            metadata: Arc::new(Mutex::new(LruCache::new(size))),
        }
    }

    pub async fn get_currency(&self, type_tag: &TypeTag) -> Result<Currency, Error> {
        let mut cache = self.metadata.lock().await;
        if !cache.contains(type_tag) {
            let mut client = self.client.clone();
            let request = GetCoinInfoRequest::default().with_coin_type(type_tag.to_string());

            let response = client
                .state_client()
                .get_coin_info(request)
                .await?
                .into_inner();

            let (symbol, decimals) = response
                .metadata
                .and_then(|m| Some((m.symbol?, m.decimals?)))
                .ok_or(MissingMetadata)?;

            let ccy = Currency {
                symbol,
                decimals: decimals as u64,
                metadata: CurrencyMetadata {
                    coin_type: type_tag.clone().to_canonical_string(true),
                },
            };
            cache.push(type_tag.clone(), ccy);
        }
        cache.get(type_tag).cloned().ok_or(MissingMetadata)
    }

    pub async fn len(&self) -> usize {
        self.metadata.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.metadata.lock().await.is_empty()
    }

    pub async fn contains(&self, type_tag: &TypeTag) -> bool {
        self.metadata.lock().await.contains(type_tag)
    }
}
