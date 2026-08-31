// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::config::Config;
use anyhow::Result;
use async_trait::async_trait;
use haneul_storage::object_store::util::fetch_checkpoint;
use haneul_types::full_checkpoint_content::CheckpointData;
use haneul_types::messages_checkpoint::CertifiedCheckpointSummary;
use std::sync::Arc;
use tracing::info;
use url::Url;

pub struct HaneulObjectStore {
    store: Arc<dyn object_store::ObjectStore>,
}

impl HaneulObjectStore {
    pub fn new(config: &Config) -> Result<Self> {
        let url = Url::parse(&config.object_store_url)?;
        let (store, _) = object_store::parse_url(&url)?;
        Ok(Self {
            store: Arc::new(store),
        })
    }

    pub async fn download_checkpoint_summary(
        &self,
        checkpoint_number: u64,
    ) -> Result<CertifiedCheckpointSummary> {
        let checkpoint = fetch_checkpoint(&self.store, checkpoint_number).await?;
        info!("Downloaded checkpoint summary: {}", checkpoint_number);
        Ok(checkpoint.summary)
    }

    pub async fn get_full_checkpoint(&self, checkpoint_number: u64) -> Result<CheckpointData> {
        let checkpoint = fetch_checkpoint(&self.store, checkpoint_number).await?;
        info!("Request full checkpoint: {}", checkpoint_number);
        Ok(CheckpointData::from(checkpoint))
    }
}

#[async_trait]
pub trait ObjectStoreExt {
    async fn get_checkpoint_summary(
        &self,
        checkpoint_number: u64,
    ) -> Result<CertifiedCheckpointSummary>;
}

#[async_trait]
impl ObjectStoreExt for HaneulObjectStore {
    async fn get_checkpoint_summary(
        &self,
        checkpoint_number: u64,
    ) -> Result<CertifiedCheckpointSummary> {
        self.download_checkpoint_summary(checkpoint_number).await
    }
}

pub async fn download_checkpoint_summary(
    config: &Config,
    checkpoint_number: u64,
) -> Result<CertifiedCheckpointSummary> {
    let store = HaneulObjectStore::new(config)?;
    store.get_checkpoint_summary(checkpoint_number).await
}
