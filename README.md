<p align="center">
<img src="docs/site/static/img/logo.svg" alt="Logo" width="100" height="100">
</p>

# Haneul

Haneul's mission is to create a financial network built on trust, where hundreds of millions of people can own and transact their money and digital assets directly, without intermediaries. Owning and trading assets inside the services you use every day, without ever having to be aware of the blockchain: that is what Haneul is aiming for. To achieve this, Haneul is designed to handle everyday transactions at that scale through an object-centric data model, the Move language, and parallel execution.

The surest path to reaching this goal was to start from technology that had already been designed and proven to hold up at that scale. Haneul began its development based on the Sui codebase created by Mysten Labs, and has deep respect for their outstanding engineering.

[![Github release](https://img.shields.io/github/v/release/GeunhwaJeong/haneul.svg?sort=semver)](https://github.com/GeunhwaJeong/haneul/releases/latest)
[![License](https://img.shields.io/github/license/GeunhwaJeong/haneul)](https://github.com/GeunhwaJeong/haneul/blob/main/LICENSE)

## Building from Source

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

## Running a Local Node

```bash
# Start a local validator with faucet
./target/release/haneul start --with-faucet --force-regenesis

# Switch to local environment
./target/release/haneul client switch --env local

# Get HANEUL tokens from faucet
./target/release/haneul client faucet

# Check balance
./target/release/haneul client gas
```

## Testing

```bash
# Unit tests
HANEUL_SKIP_SIMTESTS=1 cargo nextest run

# Test specific crate
cargo nextest run -p haneul-core

# Simulation tests
cargo simtest -p haneul-e2e-tests
```

## Linting

```bash
cargo fmt --all
cargo xclippy
```

## Project Structure

```
haneul/
├── crates/                    # Core Rust crates (haneul-core, haneul-node, haneul-types, ...)
├── consensus/                 # Mysticeti consensus engine
├── haneul-execution/          # Move VM execution layer
├── external-crates/           # Move compiler and VM
└── bridge/                    # Cross-chain bridge
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
