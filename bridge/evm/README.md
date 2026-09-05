# 🏄‍♂️ Quick Start

This project leverages [Foundry](https://github.com/foundry-rs/foundry) to manage dependencies (via soldeer), contract compilation, testing, deployment, and on chain interactions via Solidity scripting.

#### Environment configuration

Duplicate rename the `.env.example` file to `.env`. You'll need accounts and api keys for **Infura** and **Etherscan** as well as the necessary RPC URLs. Be sure to add the required values in your newly created `.env` file.

> **Note**
> The OZ foundry upgrades library uses node to verify upgrade safety. Make sure you have node version 18.17 or higher as well as npm version 10.4 or higher installed.

#### Dependencies

To install the project dependencies, run: 

```bash
forge soldeer update
```

#### Compilation

To compile your contracts, run:

```bash
forge compile
```

#### Testing

```bash
forge test
```

#### Coverage

```bash
forge coverage
```

#### Deployment

> **Note**
> Make sure the deployment config file for the target chain is created in the `deploy_configs` folder.
> The file should be named `<chainID>.json` and should have the same fields and in the same order (alphabetical) as the `example.json`.

```bash
forge script script/deploy_bridge.s.sol --rpc-url <<alias>> --broadcast --verify
```

The script reads `PRIVATE_KEY` for the deployer and honours two optional overrides:

- `OVERRIDE_CONFIG_PATH`: use a config file other than `deploy_configs/<chainID>.json`.
- `BRIDGE_IMPLEMENTATION`: contract file to install behind the `HaneulBridge` proxy. Defaults to
  `HaneulBridge.sol` (V1, which the e2e tests then upgrade to V2). Production deployments should
  set `BRIDGE_IMPLEMENTATION=HaneulBridgeV2.sol` to start on V2 directly; V2 adds no storage and
  reuses the V1 initializer.

**Ethereum mainnet (`deploy_configs/1.json`)**

`1.json` describes the Haneul mainnet bridge: source chain id 10 (Ethereum mainnet), supported
destination chain id 0 (Haneul mainnet), the committee, ETH/USDC with their Haneul-side decimals,
and the 24h USD limit. Before broadcasting:

1. Refresh `tokenPrices` (USD with 8 decimals) to current market values; they seed the limiter.
2. Fund the deployer. A full V2 deployment is about 12.8M gas across 13 transactions (measured on a mainnet fork).
3. Rehearse on a mainnet fork first (see below), then broadcast with
   `BRIDGE_IMPLEMENTATION=HaneulBridgeV2.sol` and `--verify`.
4. Record the five printed addresses and the deployment block; the bridge node config needs the
   proxy address, `eth-bridge-chain-id: 10`, and `eth-contracts-start-block-fallback` set to that block.

**Mainnet-fork rehearsal**

```bash
anvil --fork-url <<mainnet rpc>> --port 8546
PRIVATE_KEY=<<anvil key>> BRIDGE_IMPLEMENTATION=HaneulBridgeV2.sol \
  forge script script/deploy_bridge.s.sol --rpc-url http://127.0.0.1:8546 --broadcast --ffi
```

The fork has chain id 1, so the script picks up `1.json` and produces the exact gas usage of a real
deployment. Delete `broadcast/deploy_bridge.s.sol/1/` afterwards; only real broadcasts belong there.

**Local deployment**

```bash
forge script script/deploy_bridge.s.sol --fork-url anvil --broadcast
```

All deployments are saved in the `broadcast` directory.

#### External Resources

- [Writing OpenZeppelin Upgrades with Foundry](https://github.com/OpenZeppelin/openzeppelin-foundry-upgrades?tab=readme-ov-file)
- [OpenZeppelin Upgrade Requirements](https://docs.openzeppelin.com/upgrades-plugins/1.x/api-core#define-reference-contracts)
