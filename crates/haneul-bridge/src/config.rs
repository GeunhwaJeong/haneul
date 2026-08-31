// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::abi::EthBridgeConfig;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::error::BridgeError;
use crate::eth_client::EthClient;
use crate::haneul_client::HaneulBridgeClient;
use crate::metered_eth_provider::new_metered_eth_multi_provider;
use crate::metrics::BridgeMetrics;
use crate::types::{BridgeAction, is_route_valid};
use crate::utils::get_eth_contract_addresses;
use alloy::primitives::Address as EthAddress;
use alloy::providers::Provider;
use anyhow::anyhow;
use futures::StreamExt;
use haneul_config::Config;
use haneul_keys::keypair_file::read_key;
use haneul_types::base_types::ObjectRef;
use haneul_types::base_types::{HaneulAddress, ObjectID};
use haneul_types::bridge::BridgeChainId;
use haneul_types::crypto::KeypairTraits;
use haneul_types::crypto::{HaneulKeyPair, NetworkKeyPair, get_key_pair_from_rng};
use haneul_types::digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier};
use haneul_types::event::EventID;
use haneul_types::gas_coin::GasCoin;
use haneul_types::object::Owner;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EthConfig {
    /// Rpc url for Eth fullnode, used for query stuff.
    /// @deprecated (use eth_rpc_urls instead)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth_rpc_url: Option<String>,
    /// Multiple RPC URLs for Eth fullnodes.
    /// Quorum-based consensus is used across providers for redundancy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eth_rpc_urls: Option<Vec<String>>,
    /// Quorum size for multi-provider consensus. Must be <= number of URLs.
    #[serde(default = "default_quorum")]
    pub eth_rpc_quorum: usize,
    /// Health check interval in seconds for multi-provider.
    #[serde(default = "default_health_check_interval_secs")]
    pub eth_health_check_interval_secs: u64,
    /// The proxy address of HaneulBridge
    pub eth_bridge_proxy_address: String,
    /// The expected BridgeChainId on Eth side.
    pub eth_bridge_chain_id: u8,
    /// The starting block for EthSyncer to monitor eth contracts.
    /// It is required when `run_client` is true. Usually this is
    /// the block number when the bridge contracts are deployed.
    /// When BridgeNode starts, it reads the contract watermark from storage.
    /// If the watermark is not found, it will start from this fallback block number.
    /// If the watermark is found, it will start from the watermark.
    /// this v.s.`eth_contracts_start_block_override`:
    pub eth_contracts_start_block_fallback: Option<u64>,
    /// The starting block for EthSyncer to monitor eth contracts. It overrides
    /// the watermark in storage. This is useful when we want to reprocess the events
    /// from a specific block number.
    /// Note: this field has to be reset after starting the BridgeNode, otherwise it will
    /// reprocess the events from this block number every time it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth_contracts_start_block_override: Option<u64>,
}

fn default_quorum() -> usize {
    1
}

fn default_health_check_interval_secs() -> u64 {
    300 // 5 minutes
}

impl EthConfig {
    /// Backwards compatible function to get list of RPC URLs
    pub fn rpc_urls(&self) -> Vec<String> {
        if let Some(ref urls) = self.eth_rpc_urls {
            urls.clone()
        } else if let Some(ref url) = self.eth_rpc_url {
            vec![url.clone()]
        } else {
            vec![]
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct HaneulConfig {
    /// Rpc url for Haneul fullnode, used for query stuff and submit transactions.
    pub haneul_rpc_url: String,
    /// The expected BridgeChainId on Haneul side.
    pub haneul_bridge_chain_id: u8,
    /// Path of the file where bridge client key (any HaneulKeyPair) is stored.
    /// If `run_client` is true, and this is None, then use `bridge_authority_key_path` as client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_key_path: Option<PathBuf>,
    /// The gas object to use for paying for gas fees for the client. It needs to
    /// be owned by the address associated with bridge client key. If not set
    /// and `run_client` is true, it will query and use the gas object with highest
    /// amount for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_gas_object: Option<ObjectID>,
    /// Override the last processed EventID for bridge module `bridge`.
    /// When set, HaneulSyncer will start from this cursor (exclusively) instead of the one in storage.
    /// If the cursor is not found in storage or override, the query will start from genesis.
    /// Key: haneul module, Value: last processed EventID (tx_digest, event_seq).
    /// Note 1: This field should be rarely used. Only use it when you understand how to follow up.
    /// Note 2: the EventID needs to be valid, namely it must exist and matches the filter.
    /// Otherwise, it will miss one event because of fullnode Event query semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haneul_bridge_module_last_processed_event_id_override: Option<EventID>,
    /// Override the next sequence number for HaneulSyncer
    /// When set, HaneulSyncer will start from this sequence number (exclusively) instead of the one in storage.
    /// If the sequence number is not found in storage or override, the query will first fallback to the sequence number corresponding to the last processed EventID from the bridge module `bridge` (which in turn can be overridden via `haneul_bridge_module_last_processed_event_id_override`) if available, otherwise fallback to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haneul_bridge_next_sequence_number_override: Option<u64>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeNodeConfig {
    /// The port that the server listens on.
    pub server_listen_port: u16,
    /// The port that for metrics server.
    pub metrics_port: u16,
    /// Path of the file where bridge authority key (Secp256k1) is stored.
    pub bridge_authority_key_path: PathBuf,
    /// Whether to run client. If true, `haneul.bridge_client_key_path`
    /// and `db_path` needs to be provided.
    pub run_client: bool,
    /// Path of the client storage. Required when `run_client` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<PathBuf>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
    /// Haneul configuration
    pub haneul: HaneulConfig,
    /// Eth configuration
    pub eth: EthConfig,
    /// Network key used for metrics pushing
    #[serde(default = "default_ed25519_key_pair")]
    pub metrics_key_pair: NetworkKeyPair,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog_config: Option<WatchdogConfig>,
}

pub fn default_ed25519_key_pair() -> NetworkKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_interval_seconds: Option<u64>,
    pub push_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatchdogConfig {
    /// Total supplies to watch on Haneul. Mapping from coin name to coin type tag
    pub total_supplies: BTreeMap<String, String>,
}

impl Config for BridgeNodeConfig {}

impl BridgeNodeConfig {
    pub async fn validate(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(BridgeServerConfig, Option<BridgeClientConfig>)> {
        info!("Starting config validation");
        if !is_route_valid(
            BridgeChainId::try_from(self.haneul.haneul_bridge_chain_id)?,
            BridgeChainId::try_from(self.eth.eth_bridge_chain_id)?,
        ) {
            return Err(anyhow!(
                "Route between Haneul chain id {} and Eth chain id {} is not valid",
                self.haneul.haneul_bridge_chain_id,
                self.eth.eth_bridge_chain_id,
            ));
        };

        let bridge_authority_key = match read_key(&self.bridge_authority_key_path, true)? {
            HaneulKeyPair::Secp256k1(key) => key,
            _ => unreachable!("we required secp256k1 key in `read_key`"),
        };

        // we do this check here instead of `prepare_for_haneul` below because
        // that is only called when `run_client` is true.
        let haneul_client =
            Arc::new(HaneulBridgeClient::new(&self.haneul.haneul_rpc_url, metrics.clone()).await?);
        let bridge_committee = haneul_client
            .get_bridge_committee()
            .await
            .map_err(|e| anyhow!("Error getting bridge committee: {:?}", e))?;
        if !bridge_committee.is_active_member(&bridge_authority_key.public().into()) {
            return Err(anyhow!(
                "Bridge authority key is not part of bridge committee"
            ));
        }

        let (eth_client, eth_contracts) = self.prepare_for_eth(metrics.clone()).await?;
        let bridge_summary = haneul_client
            .get_bridge_summary()
            .await
            .map_err(|e| anyhow!("Error getting bridge summary: {:?}", e))?;
        if bridge_summary.chain_id != self.haneul.haneul_bridge_chain_id {
            anyhow::bail!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.haneul.haneul_bridge_chain_id,
                bridge_summary.chain_id
            );
        }

        // Validate approved actions that must be governance actions
        for action in &self.approved_governance_actions {
            if !action.is_governance_action() {
                anyhow::bail!(format!(
                    "{:?}",
                    BridgeError::ActionIsNotGovernanceAction(Box::new(action.clone()))
                ));
            }
        }
        let approved_governance_actions = self.approved_governance_actions.clone();

        let bridge_server_config = BridgeServerConfig {
            key: bridge_authority_key,
            metrics_port: self.metrics_port,
            eth_bridge_proxy_address: eth_contracts[0], // the first contract is bridge proxy
            server_listen_port: self.server_listen_port,
            haneul_client: haneul_client.clone(),
            eth_client: eth_client.clone(),
            approved_governance_actions,
        };
        if !self.run_client {
            return Ok((bridge_server_config, None));
        }

        // If client is enabled, prepare client config
        let (bridge_client_key, client_haneul_address, gas_object_ref) = self
            .prepare_for_haneul(haneul_client.clone(), metrics)
            .await?;

        let db_path = self
            .db_path
            .clone()
            .ok_or(anyhow!("`db_path` is required when `run_client` is true"))?;

        let bridge_client_config = BridgeClientConfig {
            haneul_address: client_haneul_address,
            key: bridge_client_key,
            gas_object_ref,
            metrics_port: self.metrics_port,
            haneul_client: haneul_client.clone(),
            eth_client: eth_client.clone(),
            db_path,
            eth_contracts,
            // in `prepare_for_eth` we check if this is None when `run_client` is true. Safe to unwrap here.
            eth_contracts_start_block_fallback: self
                .eth
                .eth_contracts_start_block_fallback
                .unwrap(),
            eth_contracts_start_block_override: self.eth.eth_contracts_start_block_override,
            haneul_bridge_module_last_processed_event_id_override: self
                .haneul
                .haneul_bridge_module_last_processed_event_id_override,
            haneul_bridge_next_sequence_number_override: self
                .haneul
                .haneul_bridge_next_sequence_number_override,
            haneul_bridge_chain_id: self.haneul.haneul_bridge_chain_id,
        };

        info!("Config validation complete");
        Ok((bridge_server_config, Some(bridge_client_config)))
    }

    async fn prepare_for_eth(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(Arc<EthClient>, Vec<EthAddress>)> {
        info!("Creating Ethereum client provider");
        let bridge_proxy_address = EthAddress::from_str(&self.eth.eth_bridge_proxy_address)?;
        let rpc_urls = self.eth.rpc_urls();
        anyhow::ensure!(
            !rpc_urls.is_empty(),
            "At least one Ethereum RPC URL must be provided"
        );

        let provider = new_metered_eth_multi_provider(
            rpc_urls.clone(),
            self.eth.eth_rpc_quorum,
            self.eth.eth_health_check_interval_secs,
            metrics.clone(),
        )
        .await?;

        let chain_id = provider.get_chain_id().await?;
        let (
            committee_address,
            limiter_address,
            vault_address,
            config_address,
            _weth_address,
            _usdt_address,
            _wbtc_address,
            _lbtc_address,
        ) = get_eth_contract_addresses(bridge_proxy_address, provider.clone()).await?;
        let config = EthBridgeConfig::new(config_address, provider.clone());

        if self.run_client && self.eth.eth_contracts_start_block_fallback.is_none() {
            return Err(anyhow!(
                "eth_contracts_start_block_fallback is required when run_client is true"
            ));
        }

        // If bridge chain id is Eth Mainent or Sepolia, we expect to see chain
        // identifier to match accordingly.
        let bridge_chain_id: u8 = config.chainID().call().await?;
        if self.eth.eth_bridge_chain_id != bridge_chain_id {
            return Err(anyhow!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.eth.eth_bridge_chain_id,
                bridge_chain_id
            ));
        }
        if bridge_chain_id == BridgeChainId::EthMainnet as u8 && chain_id != 1 {
            anyhow::bail!("Expected Eth chain id 1, but connected to {}", chain_id);
        }
        if bridge_chain_id == BridgeChainId::EthSepolia as u8 && chain_id != 11155111 {
            anyhow::bail!(
                "Expected Eth chain id 11155111, but connected to {}",
                chain_id
            );
        }
        info!(
            "Connected to Eth chain: {}, Bridge chain id: {}",
            chain_id, bridge_chain_id,
        );

        // Filter out zero addresses (can happen due to storage layout mismatch during upgrades)
        let all_addresses = vec![
            bridge_proxy_address,
            committee_address,
            config_address,
            limiter_address,
            vault_address,
        ];
        let valid_addresses: Vec<_> = all_addresses
            .into_iter()
            .filter(|addr| !addr.is_zero())
            .collect();

        if valid_addresses.len() < 5 {
            tracing::warn!(
                "Some contract addresses are zero - likely storage layout mismatch. \
                Event watching will be limited. Valid addresses: {:?}",
                valid_addresses
            );
        }

        let eth_client = Arc::new(
            EthClient::from_provider(provider, HashSet::from_iter(valid_addresses.clone())).await?,
        );
        info!("Ethereum client setup complete");
        Ok((eth_client, valid_addresses))
    }

    async fn prepare_for_haneul(
        &self,
        haneul_client: Arc<HaneulBridgeClient>,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(HaneulKeyPair, HaneulAddress, ObjectRef)> {
        let bridge_client_key = match &self.haneul.bridge_client_key_path {
            None => read_key(&self.bridge_authority_key_path, true),
            Some(path) => read_key(path, false),
        }?;

        // If bridge chain id is Haneul Mainent or Testnet, we expect to see chain
        // identifier to match accordingly.
        let haneul_identifier = haneul_client
            .get_chain_identifier()
            .await
            .map_err(|e| anyhow!("Error getting chain identifier from Haneul: {:?}", e))?;
        if self.haneul.haneul_bridge_chain_id == BridgeChainId::HaneulMainnet as u8
            && haneul_identifier != get_mainnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected haneul chain identifier {}, but connected to {}",
                self.haneul.haneul_bridge_chain_id,
                haneul_identifier
            );
        }
        if self.haneul.haneul_bridge_chain_id == BridgeChainId::HaneulTestnet as u8
            && haneul_identifier != get_testnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected haneul chain identifier {}, but connected to {}",
                self.haneul.haneul_bridge_chain_id,
                haneul_identifier
            );
        }
        info!(
            "Connected to Haneul chain: {}, Bridge chain id: {}",
            haneul_identifier, self.haneul.haneul_bridge_chain_id,
        );

        let client_haneul_address = HaneulAddress::from(&bridge_client_key.public());

        let gas_object_id = match self.haneul.bridge_client_gas_object {
            Some(id) => id,
            None => {
                info!("No gas object configured, finding gas object with highest balance");
                let haneul_client = haneul_rpc_api::Client::new(&self.haneul.haneul_rpc_url)?;
                // Minimum balance for gas object is 10 HANEUL
                pick_highest_balance_coin(haneul_client, client_haneul_address, 10_000_000_000)
                    .await?
            }
        };
        let (gas_coin, gas_object_ref, owner) = haneul_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;
        if owner != Owner::AddressOwner(client_haneul_address) {
            return Err(anyhow!(
                "Gas object {:?} is not owned by bridge client key's associated haneul address {:?}, but {:?}",
                gas_object_id,
                client_haneul_address,
                owner
            ));
        }
        let balance = gas_coin.value();
        info!("Gas object balance: {}", balance);
        metrics.gas_coin_balance.set(balance as i64);

        info!("Haneul client setup complete");
        Ok((bridge_client_key, client_haneul_address, gas_object_ref))
    }
}

pub struct BridgeServerConfig {
    pub key: BridgeAuthorityKeyPair,
    pub server_listen_port: u16,
    pub eth_bridge_proxy_address: EthAddress,
    pub metrics_port: u16,
    pub haneul_client: Arc<HaneulBridgeClient>,
    pub eth_client: Arc<EthClient>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
}

pub struct BridgeClientConfig {
    pub haneul_address: HaneulAddress,
    pub key: HaneulKeyPair,
    pub gas_object_ref: ObjectRef,
    pub metrics_port: u16,
    pub haneul_client: Arc<HaneulBridgeClient>,
    pub eth_client: Arc<EthClient>,
    pub db_path: PathBuf,
    pub eth_contracts: Vec<EthAddress>,
    // See `BridgeNodeConfig` for the explanation of following two fields.
    pub eth_contracts_start_block_fallback: u64,
    pub eth_contracts_start_block_override: Option<u64>,
    pub haneul_bridge_module_last_processed_event_id_override: Option<EventID>,
    pub haneul_bridge_next_sequence_number_override: Option<u64>,
    pub haneul_bridge_chain_id: u8,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeCommitteeConfig {
    pub bridge_authority_port_and_key_path: Vec<(u64, PathBuf)>,
}

impl Config for BridgeCommitteeConfig {}

pub async fn pick_highest_balance_coin(
    client: haneul_rpc_api::Client,
    address: HaneulAddress,
    minimal_amount: u64,
) -> anyhow::Result<ObjectID> {
    info!("Looking for a suitable gas coin for address {:?}", address);

    // Only look at HANEUL coins specifically
    let mut stream = client
        .list_owned_objects(address, Some(GasCoin::type_()))
        .boxed();

    let mut coins_checked = 0;

    while let Some(Ok(object)) = stream.next().await {
        let Ok(coin) = GasCoin::try_from(&object) else {
            continue;
        };
        info!(
            "Checking coin: {:?}, balance: {}",
            object.id(),
            coin.value()
        );
        coins_checked += 1;

        // Take the first coin with a sufficient balance
        if coin.value() >= minimal_amount {
            info!(
                "Found suitable gas coin with {} geunhwa (object ID: {:?})",
                coin.value(),
                object.id(),
            );
            return Ok(object.id());
        }

        // Only check a small number of coins before giving up
        if coins_checked >= 1000 {
            break;
        }
    }

    Err(anyhow!(
        "No suitable gas coin with >= {} geunhwa found for address {:?} after checking {} coins",
        minimal_amount,
        address,
        coins_checked
    ))
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EthContractAddresses {
    pub haneul_bridge: EthAddress,
    pub bridge_committee: EthAddress,
    pub bridge_config: EthAddress,
    pub bridge_limiter: EthAddress,
    pub bridge_vault: EthAddress,
}
