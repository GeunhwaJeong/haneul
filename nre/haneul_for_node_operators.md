# Haneul for Node Operators

## Overview

This document is focused on running the Haneul Node software as a Validator.

## Contents

- [Requirements](#requirements)
- [Deployment](#deployment)
- [Configuration](#configuration)
- [Connectivity](#connectivity)
- [Storage](storage.md)
- [Key Management](#key-management)
- [Monitoring](#monitoring)
  - [Logs](#logs)
  - [Metrics](#metrics)
  - [Dashboards](#dashboards)
- [Software Updates](#software-updates)
- [State Sync](#state-sync)
- [Chain Operations](#chain-operations)
- [Private Security Fixes](#private-security-fixes)

## Requirements

To run a Haneul Validator a machine with the following is required:

- CPU: 24 physical cores (or 48 virtual cores)
- Memory: 128 GB
- Storage: 4 TB NVME
- Network: 1 Gbps

## Deployment

Haneul Node can be deployed in a number of ways.

There are pre-built container images available in [Docker Hub](https://hub.docker.com/r/haneullabs/haneul-node/tags).

And pre built `linux/amd64` binaries available in S3 that can be fetched using one of the following methods:

```shell
wget https://releases.haneul.io/$HANEUL_SHA/haneul-node
```

```shell
curl https://releases.haneul.io/$HANEUL_SHA/haneul-node -o haneul-node
```

To build directly from source:

```shell
git clone https://github.com/GeunhwaJeong/haneul.git && cd haneul
git checkout [SHA|BRANCH|TAG]
cargo build --release --bin haneul-node
```

Configuration and guides are available for the following deployment options:

- [Systemd](./systemd/README.md)
- [Ansible](./ansible/README.md)
- [Docker Compose](./docker/README.md)

## Configuration

Haneul Node runs with a single configuration file provided as an argument, example:

`./haneul-node --config-path /opt/haneul/config/validator.yaml`.

Configuration templates are available here:

- [Validator](./config/validator.yaml)

## Connectivity

Haneul Node uses the following ports by default:

| protocol/port | reachability     | purpose                           |
| ------------- | ---------------- | --------------------------------- |
| TCP/8080      | inbound          | protocol / transaction interface  |
| TCP/8081      | inbound/outbound | consensus interface               |
| UDP/8081      | inbound/outbound | narwhal primary interface         |
| UDP/8082      | inbound/outbound | narwhal worker interface          |
| UDP/8084      | inbound/outbound | peer to peer state sync interface |
| TCP/8443      | outbound         | metrics pushing                   |
| TCP/9184      | localhost        | metrics scraping                  |

To run a validator successfully it is critical that ports 8080-8084 are open as outlined above, including the specific protocol (TCP/UDP).

## Storage

All Haneul Node-related data is stored by default under `/opt/haneul/db/`. This is controlled in the Haneul Node configuration file.

```shell
$ cat /opt/haneul/config/validator.yaml | grep db-path
db-path: /opt/haneul/db/authorities_db
  db-path: /opt/haneul/db/consensus_db
```

Ensure that you have an appropriately sized disk mounted for the database to write to.

- To check the size of the local Haneul Node databases:

```shell
du -sh /opt/haneul/db/
du -sh /opt/haneul/db/authorities_db
du -sh /opt/haneul/db/consensus_db
```

- To delete the local Haneul Node databases:

```shell
sudo systemctl stop haneul-node
sudo rm -rf /opt/haneul/db/authorities_db /opt/haneul/db/consensus_db
```

## Key Management

The following keys are used by Haneul Node:

| key          | scheme   | purpose                         |
| ------------ | -------- | ------------------------------- |
| protocol.key | bls12381 | transactions, narwhal consensus |
| account.key  | ed25519  | controls assets for staking     |
| network.key  | ed25519  | narwhal primary, haneul state sync |
| worker.key   | ed25519  | validate narwhal workers        |

These are configured in the [Haneul Node configuration file](#configuration).

You can generate each of these via the [haneul cli](https://docs.haneul.io/guides/developer/getting-started/haneul-install).

```
$ haneul keytool generate bls12381
$ haneul keytool generate ed25519
$ haneul keytool generate ed25519
$ haneul keytool generate ed25519
```

This will create files like `0x0061b30cdda02b6f55f575f1485a2890ec5c95b753deabbf823b6de7c936eb26.key` & `bls-0x1b7a4038f207d6c65cc106dd5be7270b3031e671fc8f9c1318b19e94a3bf3ed5.key`
which you can copy to your validator and rename to `protocol.key` or `account.key`, etc.

## Monitoring

### Metrics

Haneul Node exposes metrics via a local HTTP interface. These can be scraped for use in a central monitoring system as well as viewed directly from the node.

- View all metrics:

```shell
curl -s http://localhost:9184/metrics
```

- Search for a particular metric:

```shell
curl http://localhost:9184/metrics | grep <METRIC>
```

Haneul Node also pushes metrics to a central Haneul metrics proxy.

### Logs

Logs are controlled using the `RUST_LOG` environment variable.

The `RUST_LOG_JSON=1` environment variable can optionally be set to enable logging in JSON structured format.

Depending on your deployment method, these will be configured in the following places:

- If using Ansible, [here](./ansible/roles/haneul-node/files/haneul-node.service)
- If using Systemd natively, [here](./systemd/haneul-node.service)
- If using Docker Compose, [here](./docker/docker-compose.yaml)

To view and follow the Haneul Node logs:

```shell
journalctl -u haneul-node -f
```

To search for a particular match

```shell
journalctl -u haneul-node -g <SEARCH_TERM>
```

- If using Docker Compose, look at the examples [here](./docker/README.md#logs)

It is possible to change the logging configuration while a node is running using the admin interface.

To view the currently configured logging values:

```shell
curl localhost:1337/logging
```

To change the currently configured logging values:

```shell
curl localhost:1337/logging -d "info"
```

### Dashboards

Public dashboard for network wide visibility:

- [Haneul Testnet Validators](https://metrics.haneul.io/public-dashboards/9b841d63c9bf43fe8acec4f0fa991f5e)

For viewing total stake of validators, current active set and candidates:

- [Validators on Haneulscan](https://haneulscan.xyz/mainnet/validators)
- [Validators on HaneulVision](https://haneulvision.xyz/validators)

## Software Updates

When an update is required to the Haneul Node software the following process can be used. Follow the relevant Systemd or Docker Compose runbook depending on your deployment type. It is highly unlikely that you will want to restart with a clean database.

- If using Systemd, [here](./systemd/README.md#updates)
- If using Docker Compose, [here](./docker/README.md#updates)

## State Sync

Checkpoints in Haneul contain the permanent history of the network. They are comparable to blocks in other blockchains with one big difference being that they are lagging instead of leading. All transactions are final and executed prior to being included in a checkpoint.

These checkpoints are synchronized between validators and fullnodes via a dedicated peer to peer state sync interface.

Inter-validator state sync is always permitted however there are controls available to limit what fullnodes are allowed to sync from a specific validator.

The default and recommended `max-concurrent-connections: 0` configuration does not affect inter-validator state sync, but will restrict all fullnodes from syncing. The Haneul Node [configuration](#configuration) can be modified to allow a known fullnode to sync from a validator:

```shell
p2p-config:
  anemo-config:
    max-concurrent-connections: 0
  seed-peers:
    - address: <multiaddr>  # The p2p address of the fullnode
      peer-id: <peer-id>    # hex encoded network public key of the node
    - address: ...          # another permitted peer
      peer-id: ...
```

## Chain Operations

The following chain operations are executed using the `haneul` CLI. This binary is built and provided as a release similar to `haneul-node`, examples:

```shell
wget https://releases.haneul.io/$HANEUL_SHA/haneul
chmod +x haneul
```

```shell
curl https://releases.haneul.io/$HANEUL_SHA/haneul -o haneul
chmod +x haneul
```

It is recommended and often required that the `haneul` binary release/version matches that of the deployed network.

### Querying On-chain Metadata

Validator metadata can be queried by validator address, using `validator` subcommand of Haneul CLI:

```
haneul validator display-metadata {validator_address}
```

### Updating On-chain Metadata

You can leverage [Validator Tool](validator_tool.md) to perform majority of the following tasks.

An active/pending validator can update its on-chain metadata by submitting a transaction. Some metadata changes take effect immediately, including:

- name
- description
- image url
- project url

Other metadata (keys, addresses etc) only come into effect at the next epoch.

To update metadata, a validator makes a MoveCall transaction that interacts with the System Object. For example:

1. to update name to `new_validator_name`, use the Haneul Client CLI to call `haneul_system::update_validator_name`:

```
haneul client call --package 0x3 --module haneul_system --function update_validator_name --args 0x5 \"new_validator_name\" --gas-budget 10000
```

2. to update p2p address starting from next epoch to `/ip4/192.168.1.1`, use the Haneul Client CLI to call `haneul_system::update_validator_next_epoch_p2p_address`:

```
haneul client call --package 0x3 --module haneul_system --function update_validator_next_epoch_p2p_address --args 0x5 "[4, 192, 168, 1, 1]" --gas-budget 10000
```

See the [full list of metadata update functions here](https://github.com/GeunhwaJeong/haneul/blob/main/crates/haneul-framework/packages/haneul-system/sources/haneul_system.move#L267-L444).

### Operation Cap

To avoid touching account keys too often and allowing them to be stored off-line, validators can delegate the operation ability to another address. This address can then update the reference gas price and tallying rule on behalf of the validator.

Upon creating a `Validator`, an `UnverifiedValidatorOperationCap` is created as well and transferred to the validator address. The holder of this `Cap` object (short for "Capability") therefore could perform operational actions for this validator. To authorize another address to conduct these operations, a validator transfers the object to another address that they control. The transfer can be done by using Haneul Client CLI: `haneul client transfer`.

To rotate the delegatee address or revoke the authorization, the current holder of `Cap` transfers it to another address. In the event of compromised or lost keys, the validator could create a new `Cap` object to invalidate the incumbent one. This is done by calling `haneul_system::rotate_operation_cap`:

```
haneul client call --package 0x3 --module haneul_system --function rotate_operation_cap --args 0x5 --gas-budget 10000
```

By default the new `Cap` object is transferred to the validator address, which then could be transferred to the new delegatee address. At this point, the old `Cap` becomes invalidated and no longer represents eligibility.

To get the current valid `Cap` object's ID of a validator, use the Haneul Client CLI `haneul client objects` command after setting the holder as the active address. Or go to the [explorer](https://explorer.haneul.io/object/0x0000000000000000000000000000000000000005) and look for `operation_cap_id` of that validator in the `validators` module.

### Updating the Gas Price Survey Quote

To update the Gas Price Survey Quote of a validator, which is used to calculate the Reference Gas Price at the end of the epoch, the sender needs to hold a valid [`UnverifiedValidatorOperationCap`](#operation-cap). The sender could be the validator itself, or a trusted delegatee. To do so, call `haneul_system::request_set_gas_price`:

```
haneul client call --package 0x3 --module haneul_system --function request_set_gas_price --args 0x5 {cap_object_id} {new_gas_price} --gas-budget 10000
```

### Updating Validator Commission

To update the commission of a validator, call `haneul_system::request_set_commission_rate`, the update will effectuate in the next epoch. The sender of the transaction must be the validator, no additional objects / capabilities are required. Commission rate is expressed in basis points with 0 being 0.00%, and 10000 being 100%.

```sh
haneul client call --package 0x3 --module haneul_system --function request_set_commission_rate --args 0x5 {commission_rate}
```

If a validator is not yet in active set (candidate state), commission is updated using the `set_candidate_validator_commission_rate` function with the same arguments, like this:

```sh
haneul client call --package 0x3 --module haneul_system --function set_candidate_validator_commission_rate --args 0x5 {commission_rate}
```

### Reporting/Un-reporting Validators

To report a validator or undo an existing reporting, the sender needs to hold a valid [`UnverifiedValidatorOperationCap`](#operation-cap). The sender could be the validator itself, or a trusted delegatee. To do so, call `haneul_system::report_validator/undo_report_validator`:

```
haneul client call --package 0x3 --module haneul_system --function report_validator/undo_report_validator --args 0x5 {cap_object_id} {reportee_address} --gas-budget 10000
```

Once a validator is reported by `2f + 1` other validators by voting power, their staking rewards will be slashed.

### Joining the Validator Set

In order for a Haneul address to join the validator set, they need to first sign up as a validator candidate by calling `haneul_system::request_add_validator_candidate` with their metadata and initial configs:

```
haneul client call --package 0x3 --module haneul_system --function request_add_validator_candidate --args 0x5 {protocol_pubkey_bytes} {network_pubkey_bytes} {worker_pubkey_bytes} {proof_of_possession} {name} {description} {image_url} {project_url} {net_address}
{p2p_address} {primary_address} {worker_address} {gas_price} {commission_rate} --gas-budget 10000
```

After an address becomes a validator candidate, any address (including the candidate address itself) can start staking with the candidate's staking pool. Refer to our dedicated staking FAQ on how staking works. Once a candidate's staking pool has accumulated at least `haneul_system::MIN_VALIDATOR_JOINING_STAKE` amount of stake, the candidate can call `haneul_system::request_add_validator` to officially add themselves to the next epoch's active validator set:

```
haneul client call --package 0x3 --module haneul_system --function request_add_validator --args 0x5 --gas-budget 10000000
```

### Leaving the Validator Set

To leave the validator set starting the next epoch, the sender needs to be an active validator in the current epoch and should call `haneul_system::request_remove_validator`:

```
haneul client call --package 0x3 --module haneul_system --function request_remove_validator --args 0x5 --gas-budget 10000
```

After the validator is removed at the next epoch change, the staking pool will become inactive and stakes can only be withdrawn from an inactive pool.

## Private Security Fixes

There may be instances where urgent security fixes need to be rolled out before publicly announcing it's presence (Issues affecting liveliness, invariants such as HANEUL supply, governance etc). In order to not be actively exploited HaneulLabs will release signed security binaries incorporating such fixes with a delay in publishing the source code until a large % of our validators have patched the vulnerability.

This release process will be different and we expect us to announce the directory for such binaries out of band.
Our public key to verify these binaries would be stored [here](https://haneul-private.s3.us-west-2.amazonaws.com/haneul_security_release.pem)

You can download all the necessary signed binaries and docker artifacts incorporating the security fixes by using the [download_private.sh](https://github.com/GeunhwaJeong/haneul/blob/main/nre/download_private.sh)

Usage
`./download_private.sh <commit-sha>`

You can also download and verify specific binaries that may not be included by the above script using the [download_and_verify_private_binary.sh](https://github.com/GeunhwaJeong/haneul/blob/main/nre/download_and_verify_private_binary.sh) script.

Usage:
`./download_and_verify_private_binary.sh <commit-sha> <binary-name>`
