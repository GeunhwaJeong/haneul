// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Client-side utilities for interacting with a Haneul network from Rust.
//!
//! The JSON-RPC client this crate used to provide was removed together with
//! the fullnode JSON-RPC service. What remains is the wallet and configuration
//! layer shared by the Haneul CLI and the tooling in this repository:
//!
//! * [`wallet_context::WalletContext`] - the keystore-backed wallet used to
//!   sign and execute transactions against a network over gRPC.
//! * [`haneul_client_config::HaneulClientConfig`] - the persisted client
//!   configuration (`client.yaml`) and its named network environments.
//! * [`verify_personal_message_signature`] - verification of personal message
//!   signatures through a fullnode.
//!
//! For a general-purpose Rust SDK, use the
//! [`haneul-rust-sdk`](https://github.com/GeunhwaJeong/haneul-rust-sdk) crate.

pub use haneul_crypto;
pub use haneul_rpc;
pub use haneul_sdk_types;
pub use haneul_types as types;

pub mod digests;
pub mod error;
pub mod haneul_client_config;
pub mod verify_personal_message_signature;
pub mod wallet_context;

pub const HANEUL_COIN_TYPE: &str = "0x2::haneul::HANEUL";
pub const HANEUL_LOCAL_NETWORK_URL: &str = "http://127.0.0.1:9000";
pub const HANEUL_LOCAL_NETWORK_URL_0: &str = "http://0.0.0.0:9000";
pub const HANEUL_LOCAL_NETWORK_GAS_URL: &str = "http://127.0.0.1:5003/v2/gas";
pub const HANEUL_DEVNET_URL: &str = "https://fullnode.devnet.haneul.io:443";
pub const HANEUL_TESTNET_URL: &str = "https://fullnode.testnet.haneul.io:443";
pub const HANEUL_MAINNET_URL: &str = "https://fullnode.mainnet.haneul.io:443";
