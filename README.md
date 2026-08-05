<p align="center">
<img src="docs/site/static/img/logo.svg" alt="Logo" width="100" height="100">
</p>

# Haneul

Haneul's mission is to create a financial network built on trust, where hundreds of millions of people can own and transact their money and digital assets directly, without intermediaries. Owning and trading assets inside the services you use every day, without ever having to be aware of the blockchain: that is what Haneul is aiming for. To achieve this, Haneul is designed to handle everyday transactions at that scale through an object-centric data model, the Move language, and parallel execution.

The surest path to reaching this goal was to start from technology that had already been designed and proven to hold up at that scale. Haneul began its development based on the Sui codebase created by Mysten Labs, and has deep respect for their outstanding engineering.

[![Github release](https://img.shields.io/github/v/release/GeunhwaJeong/haneul.svg?sort=semver)](https://github.com/GeunhwaJeong/haneul/releases/latest)
[![License](https://img.shields.io/github/license/GeunhwaJeong/haneul)](https://github.com/GeunhwaJeong/haneul/blob/main/LICENSE)

**Haneul is:**

- **An object-centric blockchain**: Every asset is an object with an explicit owner, not a balance entry under an account. Transactions that touch disjoint objects execute in parallel, and transactions on single-owner objects are finalized without waiting for global ordering.
- **Programmed in Move**: Smart contracts are written in Move, a language that guarantees at the type level that assets cannot be duplicated or accidentally destroyed.
- **Secured by delegated proof of stake**: The DAG-based Mysticeti consensus provides sub-second finality. The native token HANEUL serves as both gas and the stake that secures the validator set.
- **Built for everyday users**: The protocol supports zkLogin, so people can own on-chain assets with their existing web accounts, and sponsored transactions, so applications can pay gas on their users' behalf.

## Building from Source

Building Haneul requires Rust and a handful of native dependencies. The workspace is large, so expect the first release build to take a while; incremental builds after that are much faster.

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

The required Rust version is managed automatically via [`rust-toolchain.toml`](rust-toolchain.toml).

### 2. Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update && sudo apt-get install -y \
    build-essential libssl-dev pkg-config libclang-dev cmake protobuf-compiler
```

**macOS:**
```bash
brew install cmake protobuf
```

### 3. Build

```bash
git clone https://github.com/GeunhwaJeong/haneul.git
cd haneul
cargo build --release
```

The resulting binaries end up in `target/release`.

## Executables

The build produces a number of binaries, but these are the ones you are most likely to use:

|       Command        | Description                                                                                                                                                                                                                                                                                            |
| :------------------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|     **`haneul`**     | The main CLI. It bundles everything you need day to day: `haneul start` spins up a local network, `haneul client` is the wallet and RPC client, `haneul move` builds and tests Move packages, and `haneul keytool` manages keys. Run `haneul --help` for the full list of subcommands.                   |
|    `haneul-node`     | The node daemon. It powers both validators and fullnodes; which role a node plays is determined by its configuration file.                                                                                                                                                                             |
|   `haneul-faucet`    | HTTP service that hands out test HANEUL on local networks. You will rarely run it directly, as `haneul start --with-faucet` manages one for you.                                                                                                                                                        |
|    `haneul-tool`     | Operational toolbox for node operators: database inspection, checkpoint and snapshot download, and network diagnostics.                                                                                                                                                                                |
|   `haneul-bridge`    | The bridge node that relays assets between Haneul and Ethereum.                                                                                                                                                                                                                                        |
| `haneul-indexer-alt` | Indexes chain data into Postgres and backs the GraphQL and JSON-RPC services.                                                                                                                                                                                                                          |

## Running a Local Node

The `haneul` binary can spin up a complete local network on your machine: a single validator with a faucet attached. This is the fastest way to try things out.

```bash
./target/release/haneul start --with-faucet --force-regenesis
```

`--force-regenesis` starts from a fresh genesis every time, so nothing persists between runs. Once the node is up, open another terminal and point the client at your local network:

```bash
./target/release/haneul client switch --env local
./target/release/haneul client faucet
./target/release/haneul client gas
```

The faucet grants test HANEUL, and `client gas` lists the coin objects your address now owns.

## Testing

Most tests are ordinary Rust tests, run through [cargo-nextest](https://nexte.st/):

```bash
HANEUL_SKIP_SIMTESTS=1 cargo nextest run

# or just one crate
cargo nextest run -p haneul-core
```

Consensus and end-to-end tests additionally run under a deterministic simulator that controls time and the network, so that distributed-systems bugs reproduce reliably instead of flaking. These must go through `cargo simtest`:

```bash
cargo simtest -p haneul-e2e-tests
```

## Linting

`cargo xclippy` is the workspace's clippy wrapper with our lint configuration. CI runs both of these, so running them before pushing saves a round trip:

```bash
cargo fmt --all
cargo xclippy
```

## Contributing

Thank you for considering helping out with the source code! We welcome contributions from anyone on the internet, and are grateful for even the smallest of fixes!

If you'd like to contribute to Haneul, please fork, fix, commit and send a pull request for the maintainers to review and merge into the main code base. If you wish to submit more complex changes though, please [open an issue](https://github.com/GeunhwaJeong/haneul/issues) first to ensure those changes are in line with the general philosophy of the project and/or get some early feedback which can make both your efforts much lighter as well as our review and merge procedures quick and simple.

Please make sure your contributions adhere to our coding guidelines:

* Code must be formatted with `cargo fmt` and must pass `cargo xclippy` without warnings.
* Pull requests need to be based on and opened against the `main` branch.
* Commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format, prefixed with the area they modify.
  * E.g. "fix(name-service): point mainnet config at the deployed objects"

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more details on configuring your environment, managing project dependencies, and testing procedures.

## License

See the [LICENSE](LICENSE) file for more details.
