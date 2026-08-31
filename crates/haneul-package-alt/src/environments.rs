// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_sdk::types::{
    digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier},
    supported_protocol_versions::Chain,
};
use move_package_alt::schema::Environment;

pub fn testnet_environment() -> Environment {
    Environment {
        name: Chain::Testnet.as_str().to_string(),
        id: get_testnet_chain_identifier().to_string(),
    }
}

pub fn mainnet_environment() -> Environment {
    Environment {
        name: Chain::Mainnet.as_str().to_string(),
        id: get_mainnet_chain_identifier().to_string(),
    }
}
