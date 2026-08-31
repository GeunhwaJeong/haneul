// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_sdk::HaneulClientBuilder;

// This example shows the few basic ways to connect to a Haneul network.
// There are several in-built methods for connecting to the
// Haneul devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no Haneul network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Haneul local network version: {}", haneul.api_version());

    // local Haneul network, like the above one but using the dedicated function
    let haneul_local = HaneulClientBuilder::default().build_localnet().await?;
    println!(
        "Haneul local network version: {}",
        haneul_local.api_version()
    );

    // Haneul devnet -- https://fullnode.devnet.haneul.io:443
    let haneul_devnet = HaneulClientBuilder::default().build_devnet().await?;
    println!("Haneul devnet version: {}", haneul_devnet.api_version());

    // Haneul testnet -- https://fullnode.testnet.haneul.io:443
    let haneul_testnet = HaneulClientBuilder::default().build_testnet().await?;
    println!("Haneul testnet version: {}", haneul_testnet.api_version());

    // Haneul mainnet -- https://fullnode.mainnet.haneul.io:443
    let haneul_mainnet = HaneulClientBuilder::default().build_mainnet().await?;
    println!("Haneul mainnet version: {}", haneul_mainnet.api_version());

    println!("rpc methods: {:?}", haneul_testnet.available_rpc_methods());
    println!(
        "available subscriptions: {:?}",
        haneul_testnet.available_subscriptions()
    );

    Ok(())
}
