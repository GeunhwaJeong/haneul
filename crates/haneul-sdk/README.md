This crate provides the Haneul Rust SDK, containing APIs to interact with the Haneul network. Auto-generated documentation for this crate is [here](https://haneullabs.github.io/haneul/haneul_sdk/index.html).

## Getting started

Add the `haneul-sdk` dependency as following:

```toml
haneul_sdk = { git = "https://github.com/GeunhwaJeong/haneul", package = "haneul-sdk"}
tokio = { version = "1.2", features = ["full"] }
anyhow = "1.0"
```

The main building block for the Haneul Rust SDK is the `HaneulClientBuilder`, which provides a simple and straightforward way of connecting to a Haneul network and having access to the different available APIs.

In the following example, the application connects to the Haneul `testnet` and `devnet` networks and prints out their respective RPC API versions.

```rust
use haneul_sdk::HaneulClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Haneul testnet -- https://fullnode.testnet.haneul.io:443
    let haneul_testnet = HaneulClientBuilder::default().build_testnet().await?;
    println!("Haneul testnet version: {}", haneul_testnet.api_version());

     // Haneul devnet -- https://fullnode.devnet.haneul.io:443
    let haneul_devnet = HaneulClientBuilder::default().build_devnet().await?;
    println!("Haneul devnet version: {}", haneul_devnet.api_version());

    // Haneul mainnet -- https://fullnode.mainnet.haneul.io:443
    let haneul_mainnet = HaneulClientBuilder::default().build_mainnet().await?;
    println!("Haneul mainnet version: {}", haneul_mainnet.api_version());

    Ok(())
}

```

## Documentation for haneul-sdk crate

[GitHub Pages](https://haneullabs.github.io/haneul/haneul_sdk/index.html) hosts the generated documentation for all Rust crates in the Haneul repository.

### Building documentation locally

You can also build the documentation locally. To do so,

1. Clone the `haneul` repo locally. Open a Terminal or Console and go to the `haneul/crates/haneul-sdk` directory.

1. Run `cargo doc` to build the documentation into the `haneul/target` directory. Take note of location of the generated file from the last line of the output, for example `Generated /Users/foo/haneul/target/doc/haneul_sdk/index.html`.

1. Use a web browser, like Chrome, to open the `.../target/doc/haneul_sdk/index.html` file at the location your console reported in the previous step.

## Rust SDK examples

The [examples](https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-sdk/examples) folder provides both basic and advanced examples.

There are serveral files ending in `_api.rs` which provide code examples of the corresponding APIs and their methods. These showcase how to use the Haneul Rust SDK, and can be run against the Haneul testnet. Below are instructions on the prerequisites and how to run these examples.

### Prerequisites

Unless otherwise specified, most of these examples assume `Rust` and `cargo` are installed, and that there is an available internet connection. The examples connect to the Haneul testnet (`https://fullnode.testnet.haneul.io:443`) and execute different APIs using the active address from the local wallet. If there is no local wallet, it will create one, generate two addresses, set one of them to be active, and it will request 1 HANEUL from the testnet faucet for the active address.

### Running the existing examples

In the root folder of the `haneul` repository (or in the `haneul-sdk` crate folder), you can individually run examples using the command  `cargo run --example filename` (without `.rs` extension). For example:
* `cargo run --example haneul_client` -- this one requires a local Haneul network running (see [here](#Connecting to Haneul Network
)). If you do not have a local Haneul network running, please skip this example.
* `cargo run --example coin_read_api`
* `cargo run --example event_api` -- note that this will subscribe to a stream and thus the program will not terminate unless forced (Ctrl+C)
* `cargo run --example governance_api`
* `cargo run --example read_api`
* `cargo run --example programmable_transactions_api`
* `cargo run --example sign_tx_guide`

### Basic Examples

#### Connecting to Haneul Network
The `HaneulClientBuilder` struct provides a connection to the JSON-RPC server that you use for all read-only operations. The default URLs to connect to the Haneul network are:

- Local: http://127.0.0.1:9000
- Devnet: https://fullnode.devnet.haneul.io:443
- Testnet: https://fullnode.testnet.haneul.io:443
- Mainnet: https://fullnode.mainnet.haneul.io:443

For all available servers, see [here](https://haneul.io/networkinfo).

For running a local Haneul network, please follow [this guide](https://docs.haneul.io/build/haneul-local-network) for installing Haneul and [this guide](https://docs.haneul.io/build/haneul-local-network#start-the-local-network) for starting the local Haneul network.


```rust
use haneul_sdk::HaneulClientBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let haneul = HaneulClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("Haneul local network version: {}", haneul.api_version());

    // local Haneul network, like the above one but using the dedicated function
    let haneul_local = HaneulClientBuilder::default().build_localnet().await?;
    println!("Haneul local network version: {}", haneul_local.api_version());

    // Haneul devnet -- https://fullnode.devnet.haneul.io:443
    let haneul_devnet = HaneulClientBuilder::default().build_devnet().await?;
    println!("Haneul devnet version: {}", haneul_devnet.api_version());

    // Haneul testnet -- https://fullnode.testnet.haneul.io:443
    let haneul_testnet = HaneulClientBuilder::default().build_testnet().await?;
    println!("Haneul testnet version: {}", haneul_testnet.api_version());

    Ok(())
}
```

#### Read the total coin balance for each coin type owned by this address
```rust
use std::str::FromStr;
use haneul_sdk::types::base_types::HaneulAddress;
use haneul_sdk::{ HaneulClientBuilder};
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

   let haneul_local = HaneulClientBuilder::default().build_localnet().await?;
   println!("Haneul local network version: {}", haneul_local.api_version());

   let active_address = HaneulAddress::from_str("<YOUR HANEUL ADDRESS>")?; // change to your Haneul address

   let total_balance = haneul_local
      .coin_read_api()
      .get_all_balances(active_address)
      .await?;
   println!("The balances for all coins owned by address: {active_address} are {:#?}", total_balance);
   Ok(())
}
```

## Advanced examples

See the programmable transactions [example](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-sdk/examples/programmable_transactions_api.rs).

## Games examples

### Tic Tac Toe quick start

1. Prepare the environment
   1. Install `haneul` binary following the [Haneul installation](https://github.com/GeunhwaJeong/haneul/blob/main/docs/content/guides/developer/getting-started/haneul-install.mdx) docs.
   1. [Connect to Haneul Devnet](https://github.com/GeunhwaJeong/haneul/blob/main/docs/content/guides/developer/getting-started/connect.mdx).
   1. [Make sure you have two addresses with gas](https://github.com/GeunhwaJeong/haneul/blob/main/docs/content/guides/developer/getting-started/get-address.mdx) by using the `new-address` command to create new addresses:
      ```shell
      haneul client new-address ed25519
      ```
      You must specify the key scheme, one of `ed25519` or `secp256k1` or `secp256r1`.
      You can skip this step if you are going to play with a friend. :)
   1. [Request Haneul tokens](https://github.com/GeunhwaJeong/haneul/blob/main/docs/content/guides/developer/getting-started/get-coins.mdx) for all addresses that will be used to join the game.

2. Publish the move contract
   1. [Download the Haneul source code](https://github.com/GeunhwaJeong/haneul/blob/main/docs/content/guides/developer/getting-started/haneul-install.mdx).
   1. Publish the [`tic-tac-toe` package](https://github.com/GeunhwaJeong/haneul/tree/main/examples/tic-tac-toe/move)
      using the Haneul client:
      ```shell
      haneul client publish --path /path-to-haneul-source-code/examples/tic-tac-toe/move
      ```
   1. Record the package object ID.

3. Create a new tic-tac-toe game
   1. Run the following command in the [`tic-tac-toe/cli` directory](https://github.com/GeunhwaJeong/haneul/tree/main/examples/tic-tac-toe/cli) to start a new game, replacing the game package objects ID with the one you recorded:
      ```shell
      cargo run -- new --package-id <<tic-tac-toe package object ID>> <<player O address>>
      ```
      This will create a game between the active address in the keystore, and the specified Player O.
   1. Copy the game ID and pass it to your friend to join the game.

4. Making a move

   Run the following command in the [`tic-tac-toe/cli` directory](https://github.com/GeunhwaJeong/haneul/tree/main/examples/tic-tac-toe/cli) to make a move in an existing game, as the active address in the CLI, replacing the game ID and address accordingly:
   ```shell
   cargo run -- move --package-id <<tic-tac-toe package object ID>> --row $R --col $C <<game ID>>
   ```

## License

[SPDX-License-Identifier: Apache-2.0](https://github.com/GeunhwaJeong/haneul/blob/main/LICENSE)
