// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

mod utils;
use futures::stream::StreamExt;
use haneul_sdk::HaneulClientBuilder;
use haneul_sdk::rpc_types::EventFilter;
use utils::{setup_for_write, split_coin_digest};

// This example showcases how to use the Event API.
// At the end of the program it subscribes to the events
// on the Haneul testnet and prints every incoming event to
// the console. The program will loop until it is force
// stopped.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (haneul, active_address, _second_address) = setup_for_write().await?;

    println!(" *** Get events *** ");
    // for demonstration purposes, we set to make a transaction
    let digest = split_coin_digest(&haneul, &active_address).await?;
    let events = haneul.event_api().get_events(digest).await?;
    println!("{:?}", events);
    println!(" *** Get events ***\n ");

    let descending = true;
    let query_events = haneul
        .event_api()
        .query_events(EventFilter::All([]), None, Some(5), descending) // query first 5 events in descending order
        .await?;
    println!(" *** Query events *** ");
    println!("{:?}", query_events);
    println!(" *** Query events ***\n ");

    let ws = HaneulClientBuilder::default()
        .ws_url("wss://rpc.testnet.haneul.io:443")
        .build("https://fullnode.testnet.haneul.io:443")
        .await?;
    println!("WS version {:?}", ws.api_version());

    let mut subscribe = ws.event_api().subscribe_event(EventFilter::All([])).await?;

    loop {
        println!("{:?}", subscribe.next().await);
    }
}
