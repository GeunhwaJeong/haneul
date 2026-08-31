// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Network node configuration for Haneul data stores.
//!
//! Defines the [`Node`] enum for specifying which Haneul network to connect to
//! (mainnet, testnet, or custom) and provides URL resolution for both
//! GraphQL and JSON-RPC endpoints.

use haneul_types::supported_protocol_versions::Chain;
use std::str::FromStr;

/// GraphQL endpoint for Haneul mainnet.
pub const MAINNET_GQL_URL: &str = "https://graphql.mainnet.haneul.io/graphql";
/// GraphQL endpoint for Haneul testnet.
pub const TESTNET_GQL_URL: &str = "https://graphql.testnet.haneul.io/graphql";
/// JSON-RPC endpoint for Haneul mainnet.
pub const MAINNET_RPC_URL: &str = "https://fullnode.mainnet.haneul.io:443";
/// JSON-RPC endpoint for Haneul testnet.
pub const TESTNET_RPC_URL: &str = "https://fullnode.testnet.haneul.io:443";

/// Represents a Haneul network node configuration.
///
/// Used to specify which network the data store should connect to.
#[derive(Clone, Debug)]
pub enum Node {
    /// Haneul mainnet
    Mainnet,
    /// Haneul testnet
    Testnet,
    /// Custom network with a user-provided URL
    Custom(String),
}

impl Node {
    /// Returns the [`Chain`] identifier for this node.
    pub fn chain(&self) -> Chain {
        match self {
            Node::Mainnet => Chain::Mainnet,
            Node::Testnet => Chain::Testnet,
            Node::Custom(_) => Chain::Unknown,
        }
    }

    /// Returns a human-readable network name.
    pub fn network_name(&self) -> String {
        match self {
            Node::Mainnet => "mainnet".to_string(),
            Node::Testnet => "testnet".to_string(),
            Node::Custom(url) => url.clone(),
        }
    }

    /// Returns the GraphQL endpoint URL for this node.
    pub fn gql_url(&self) -> &str {
        match self {
            Node::Mainnet => MAINNET_GQL_URL,
            Node::Testnet => TESTNET_GQL_URL,
            // For custom, assume it's already a GraphQL URL
            Node::Custom(url) => url.as_str(),
        }
    }

    /// Returns the JSON-RPC endpoint URL for this node.
    pub fn node_url(&self) -> &str {
        match self {
            Node::Mainnet => MAINNET_RPC_URL,
            Node::Testnet => TESTNET_RPC_URL,
            // For custom, assume it's already an RPC URL
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
            _ => Ok(Node::Custom(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_gql_url_returns_provided_url() {
        let url = "https://graphql.devnet.example.com/graphql";
        let node = Node::Custom(url.to_string());
        assert_eq!(node.gql_url(), url);
    }

    #[test]
    fn from_str_unknown_becomes_custom() {
        let url = "https://graphql.devnet.example.com/graphql";
        match Node::from_str(url).unwrap() {
            Node::Custom(s) => assert_eq!(s, url),
            other => panic!("expected Custom, got {other:?}"),
        }
    }
}
