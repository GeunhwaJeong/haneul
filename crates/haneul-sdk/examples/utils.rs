// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::{str::FromStr, time::Duration};

use anyhow::bail;
use futures::{future, stream::StreamExt};
use haneul_config::{
    Config, HANEUL_CLIENT_CONFIG, HANEUL_KEYSTORE_FILENAME, PersistedConfig, haneul_config_dir,
};
use haneul_json_rpc_types::{Coin, HaneulObjectDataOptions};
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, GenerateOptions};
use haneul_sdk::{
    haneul_client_config::{HaneulClientConfig, HaneulEnv},
    wallet_context::WalletContext,
};
use tracing::info;

use haneul_sdk::types::{
    base_types::{HaneulAddress, ObjectID},
    digests::TransactionDigest,
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::{Argument, Command, Transaction, TransactionData},
    transaction_driver_types::ExecuteTransactionRequestType,
};
use reqwest::Client;
use serde_json::json;
use shared_crypto::intent::Intent;

use haneul_sdk::{
    HaneulClient, HaneulClientBuilder, rpc_types::HaneulTransactionBlockResponseOptions,
};

#[derive(serde::Deserialize)]
struct FaucetResponse {
    task: String,
    error: Option<String>,
}

// const HANEUL_FAUCET: &str = "https://faucet.devnet.haneul.io/v2/gas"; // devnet faucet

// Testnet faucet is under heavy rate limit, we recommend using devnet for these examples
pub const HANEUL_FAUCET: &str = "https://faucet.testnet.haneul.io/v2/gas"; // testnet faucet

// const HANEUL_FAUCET: &str = "http://127.0.0.1:9123/v2/gas";

/// Return a haneul client to interact with the APIs,
/// the active address of the local wallet, and another address that can be used as a recipient.
///
/// By default, this function will set up a wallet locally if there isn't any, or reuse the
/// existing one and its active address. This function should be used when two addresses are needed,
/// e.g., transferring objects from one address to another.
pub async fn setup_for_write() -> Result<(HaneulClient, HaneulAddress, HaneulAddress), anyhow::Error>
{
    let (client, active_address) = setup_for_read().await?;
    // make sure we have some HANEUL (5_000_000 GEUNHWA) on this address
    let coin = fetch_coin(&client, &active_address).await?;
    if coin.is_none() {
        request_tokens_from_faucet(active_address, &client).await?;
    }
    let wallet = retrieve_wallet().await?;
    let addresses = wallet.get_addresses();
    let addresses = addresses
        .into_iter()
        .filter(|address| address != &active_address)
        .collect::<Vec<_>>();
    let recipient = addresses
        .first()
        .expect("Cannot get the recipient address needed for writing operations. Aborting");

    Ok((client, active_address, *recipient))
}

/// Return a haneul client to interact with the APIs and an active address from the local wallet.
///
/// This function sets up a wallet in case there is no wallet locally,
/// and ensures that the active address of the wallet has HANEUL on it.
/// If there is no HANEUL owned by the active address, then it will request
/// HANEUL from the faucet.
pub async fn setup_for_read() -> Result<(HaneulClient, HaneulAddress), anyhow::Error> {
    let client = HaneulClientBuilder::default().build_testnet().await?;
    println!("Haneul testnet version is: {}", client.api_version());
    let mut wallet = retrieve_wallet().await?;
    assert!(wallet.get_addresses().len() >= 2);
    let active_address = wallet.active_address()?;

    println!("Wallet active address is: {active_address}");
    Ok((client, active_address))
}

/// Request tokens from the Faucet for the given address
#[allow(unused_assignments)]
pub async fn request_tokens_from_faucet(
    address: HaneulAddress,
    haneul_client: &HaneulClient,
) -> Result<(), anyhow::Error> {
    let address_str = address.to_string();
    let json_body = json![{
        "FixedAmountRequest": {
            "recipient": &address_str
        }
    }];

    // make the request to the faucet JSON RPC API for coin
    let client = Client::new();
    let resp = client
        .post(HANEUL_FAUCET)
        .header("Content-Type", "application/json")
        .json(&json_body)
        .send()
        .await?;
    println!(
        "Faucet request for address {address_str} has status: {}",
        resp.status()
    );
    println!("Waiting for the faucet to complete the gas request...");
    let faucet_resp: FaucetResponse = resp.json().await?;

    let task_id = if let Some(err) = faucet_resp.error {
        bail!("Faucet request was unsuccessful. Error is {err:?}")
    } else {
        faucet_resp.task
    };

    println!("Faucet request task id: {task_id}");

    let json_body = json![{
        "GetBatchSendStatusRequest": {
            "task_id": &task_id
        }
    }];

    let mut coin_id = "".to_string();

    // wait for the faucet to finish the batch of token requests
    loop {
        let resp = client
            .get("https://faucet.testnet.haneul.io/v1/status")
            .header("Content-Type", "application/json")
            .json(&json_body)
            .send()
            .await?;
        let text = resp.text().await?;
        if text.contains("SUCCEEDED") {
            let resp_json: serde_json::Value = serde_json::from_str(&text).unwrap();

            coin_id = <&str>::clone(
                &resp_json
                    .pointer("/status/transferred_gas_objects/sent/0/id")
                    .unwrap()
                    .as_str()
                    .unwrap(),
            )
            .to_string();

            break;
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    // wait until the fullnode has the coin object, and check if it has the same owner
    loop {
        let owner = haneul_client
            .read_api()
            .get_object_with_options(
                ObjectID::from_str(&coin_id)?,
                HaneulObjectDataOptions::new().with_owner(),
            )
            .await?;

        if owner.owner().is_some() {
            let owner_address = owner.owner().unwrap().get_owner_address()?;
            if owner_address == address {
                break;
            }
        } else {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    Ok(())
}

/// Return the coin owned by the address that has at least 5_000_000 GEUNHWA, otherwise returns None
pub async fn fetch_coin(
    haneul: &HaneulClient,
    sender: &HaneulAddress,
) -> Result<Option<Coin>, anyhow::Error> {
    let coin_type = "0x2::haneul::HANEUL".to_string();
    let coins_stream = haneul
        .coin_read_api()
        .get_coins_stream(*sender, Some(coin_type));

    let mut coins = coins_stream
        .skip_while(|c| future::ready(c.balance < 5_000_000))
        .boxed();
    let coin = coins.next().await;
    Ok(coin)
}

/// Return a transaction digest from a split coin + merge coins transaction
pub async fn split_coin_digest(
    haneul: &HaneulClient,
    sender: &HaneulAddress,
) -> Result<TransactionDigest, anyhow::Error> {
    let coin = match fetch_coin(haneul, sender).await? {
        None => {
            request_tokens_from_faucet(*sender, haneul).await?;
            fetch_coin(haneul, sender)
                .await?
                .expect("Supposed to get a coin with HANEUL, but didn't. Aborting")
        }
        Some(c) => c,
    };

    println!(
        "Address: {sender}. The selected coin for split is {} and has a balance of {}\n",
        coin.coin_object_id, coin.balance
    );

    // set the maximum gas budget
    let max_gas_budget = 5_000_000;

    // get the reference gas price from the network
    let gas_price = haneul.read_api().get_reference_gas_price().await?;

    // now we programmatically build the transaction through several commands
    let mut ptb = ProgrammableTransactionBuilder::new();
    // first, we want to split the coin, and we specify how much HANEUL (in GEUNHWA) we want
    // for the new coin
    let split_coin_amount = ptb.pure(1000u64)?; // note that we need to specify the u64 type here
    ptb.command(Command::SplitCoins(
        Argument::GasCoin,
        vec![split_coin_amount],
    ));
    // now we want to merge the coins (so that we don't have many coins with very small values)
    // observe here that we pass Argument::Result(0), which instructs the PTB to get
    // the result from the previous command
    ptb.command(Command::MergeCoins(
        Argument::GasCoin,
        vec![Argument::Result(0)],
    ));

    // we finished constructing our PTB and we need to call finish
    let builder = ptb.finish();

    // using the PTB that we just constructed, create the transaction data
    // that we will submit to the network
    let tx_data = TransactionData::new_programmable(
        *sender,
        vec![coin.object_ref()],
        builder,
        max_gas_budget,
        gas_price,
    );

    // sign & execute the transaction
    let keystore =
        FileBasedKeystore::load_or_create(&haneul_config_dir()?.join(HANEUL_KEYSTORE_FILENAME))?;
    let signature = keystore
        .sign_secure(sender, &tx_data, Intent::haneul_transaction())
        .await?;

    let transaction_response = haneul
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            HaneulTransactionBlockResponseOptions::new(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    Ok(transaction_response.digest)
}

pub async fn retrieve_wallet() -> Result<WalletContext, anyhow::Error> {
    let wallet_conf = haneul_config_dir()?.join(HANEUL_CLIENT_CONFIG);
    let keystore_path = haneul_config_dir()?.join(HANEUL_KEYSTORE_FILENAME);

    // check if a wallet exists and if not, create a wallet and a haneul client config
    if !keystore_path.exists() {
        let keystore = FileBasedKeystore::load_or_create(&keystore_path)?;
        keystore.save().await?;
    }

    if !wallet_conf.exists() {
        let keystore = FileBasedKeystore::load_or_create(&keystore_path)?;
        let mut client_config = HaneulClientConfig::new(keystore.into());

        client_config.add_env(HaneulEnv::testnet());
        client_config.add_env(HaneulEnv::devnet());
        client_config.add_env(HaneulEnv::localnet());

        if client_config.active_env.is_none() {
            client_config.active_env = client_config.envs.first().map(|env| env.alias.clone());
        }

        client_config.save(&wallet_conf)?;
        info!("Client config file is stored in {:?}.", &wallet_conf);
    }

    let mut keystore = FileBasedKeystore::load_or_create(&keystore_path)?;
    let mut client_config: HaneulClientConfig = PersistedConfig::read(&wallet_conf)?;

    if client_config.active_address.is_none() {
        let default_active_address = if let Some(address) = keystore.addresses().first() {
            *address
        } else {
            keystore
                .generate(None, GenerateOptions::default())
                .await?
                .address
        };

        client_config.active_address = Some(default_active_address);
    }

    if keystore.addresses().len() < 2 {
        keystore.generate(None, GenerateOptions::default()).await?;
    }

    client_config.save(&wallet_conf)?;

    let wallet =
        WalletContext::new(&wallet_conf)?.with_request_timeout(std::time::Duration::from_secs(60));

    Ok(wallet)
}

// this function should not be used. It is only used to make clippy happy,
// and to reduce the number of allow(dead_code) annotations to just this one
#[allow(dead_code)]
async fn just_for_clippy() -> Result<(), anyhow::Error> {
    let (haneul, sender, _recipient) = setup_for_write().await?;
    let _digest = split_coin_digest(&haneul, &sender).await?;
    Ok(())
}
