// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Network node configuration for Haneul data stores.
//!
//! Defines the [`Node`] enum for specifying which Haneul network to connect to (mainnet, testnet,
//! devnet, or custom) and provides URL resolution for both GraphQL and gRPC endpoints.

use std::str::FromStr;

use haneul_types::supported_protocol_versions::Chain;

/// GraphQL endpoint for Haneul mainnet.
pub(crate) const MAINNET_GQL_URL: &str = "https://graphql.mainnet.haneul.io/graphql";
/// GraphQL endpoint for Haneul testnet.
pub(crate) const TESTNET_GQL_URL: &str = "https://graphql.testnet.haneul.io/graphql";
/// GraphQL endpoint for Haneul devnet.
pub(crate) const DEVNET_GQL_URL: &str = "https://graphql.devnet.haneul.io/graphql";

/// Represents a Haneul network node configuration.
///
/// Used to specify which network the data store should connect to.
#[derive(Clone, Debug)]
pub enum Node {
    /// Haneul mainnet
    Mainnet,
    /// Haneul testnet
    Testnet,
    /// Haneul devnet
    Devnet,
    /// Custom network with a user-provided URL
    Custom(String),
}

impl Node {
    /// Returns the [`Chain`] identifier for this node.
    pub fn chain(&self) -> Chain {
        match self {
            Node::Mainnet => Chain::Mainnet,
            Node::Testnet => Chain::Testnet,
            Node::Devnet => Chain::Unknown,
            Node::Custom(_) => Chain::Unknown,
        }
    }

    /// Returns a human-readable network name.
    pub fn network_name(&self) -> String {
        match self {
            Node::Mainnet => "mainnet".to_string(),
            Node::Testnet => "testnet".to_string(),
            Node::Devnet => "devnet".to_string(),
            Node::Custom(url) => url.clone(),
        }
    }

    /// Returns the GraphQL endpoint URL for this node.
    pub(crate) fn gql_url(&self) -> &str {
        match self {
            Node::Mainnet => MAINNET_GQL_URL,
            Node::Testnet => TESTNET_GQL_URL,
            Node::Devnet => DEVNET_GQL_URL,
            Node::Custom(url) => url.as_str(),
        }
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainnet" => Ok(Node::Mainnet),
            "testnet" => Ok(Node::Testnet),
            "devnet" => Ok(Node::Devnet),
            _ => Ok(Node::Custom(s.to_string())),
        }
    }
}
