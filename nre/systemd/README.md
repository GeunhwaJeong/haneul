# Run a Haneul Node using Systemd

Tested using:
- Ubuntu 20.04 (linux/amd64) on bare metal
- Ubuntu 22.04 (linux/amd64) on bare metal

## Prerequisites and Setup

1. Add a `haneul` user and the `/opt/haneul` directories

```shell
sudo useradd haneul
sudo mkdir -p /opt/haneul/bin
sudo mkdir -p /opt/haneul/config
sudo mkdir -p /opt/haneul/db
sudo mkdir -p /opt/haneul/key-pairs
sudo chown -R haneul:haneul /opt/haneul
```

2. Install the Haneul Node (haneul-node) binary, two options:
    
- Pre-built binary stored in Amazon S3:
        
```shell
wget https://releases.haneul.io/$HANEUL_SHA/haneul-node
chmod +x haneul-node
sudo mv haneul-node /opt/haneul/bin
```

- Build from source:

```shell
git clone https://github.com/GeunhwaJeong/haneul.git && cd haneul
git checkout $HANEUL_SHA
cargo build --release --bin haneul-node
mv ./target/release/haneul-node /opt/haneul/bin/haneul-node
```

3. Copy your key-pairs into `/opt/haneul/key-pairs/` 

If generated during the Genesis ceremony these will be at `HaneulExternal.git/haneul-testnet-wave3/genesis/key-pairs/`

Make sure when you copy them they retain `haneul` user permissions. To be safe you can re-run: `sudo chown -R haneul:haneul /opt/haneul`

4. Update the node configuration file and place it in the `/opt/haneul/config/` directory.

Add the paths to your private keys to validator.yaml. If you chose to put them in `/opt/haneul/key-pairs`, you can use the following example: 

```
protocol-key-pair: 
  path: /opt/haneul/key-pairs/protocol.key
worker-key-pair: 
  path: /opt/haneul/key-pairs/worker.key
network-key-pair: 
  path: /opt/haneul/key-pairs/network.key
```

5. Place genesis.blob in `/opt/haneul/config/` (should be available after the Genesis ceremony)

6. Copy the haneul-node systemd service unit file 

File: [haneul-node.service](./haneul-node.service)

Copy the file to `/etc/systemd/system/haneul-node.service`.

7. Reload systemd with this new service unit file, run:

```shell
sudo systemctl daemon-reload
```

8. Enable the new service with systemd

```shell
sudo systemctl enable haneul-node.service
```

## Connectivity

You may need to explicitly open the ports outlined in [Haneul for Node Operators](../haneul_for_node_operators.md#connectivity) for the required Haneul Node connectivity.

## Start the node

Start the Validator:

```shell
sudo systemctl start haneul-node
```

Check that the node is up and running:

```shell
sudo systemctl status haneul-node
```

Follow the logs with:

```shell
journalctl -u haneul-node -f
```

## Updates

When an update is required to the Haneul Node software the following procedure can be used. It is highly **unlikely** that you will want to restart with a clean database.

- assumes haneul-node lives in `/opt/haneul/bin/`
- assumes systemd service is named haneul-node
- **DO NOT** delete the Haneul databases

1. Stop haneul-node systemd service

```
sudo systemctl stop haneul-node
```

2. Fetch the new haneul-node binary

```shell
wget https://releases.haneul.io/${HANEUL_SHA}/haneul-node
```

3. Update and move the new binary:

```
chmod +x haneul-node
sudo mv haneul-node /opt/haneul/bin/
```

4. start haneul-node systemd service

```
sudo systemctl start haneul-node
```
