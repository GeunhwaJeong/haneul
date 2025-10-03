#!/bin/bash
# Local Haneul Blockchain Setup Script
# This script initializes and runs a local Haneul node for development

set -e

# Configuration
export CHAIN_ID="${CHAIN_ID:-local-1}"
export MONIKER="${MONIKER:-haneul-local}"
export KEYRING="${KEYRING:-test}"
export HOME_DIR=$(eval echo "${HOME_DIR:-"~/.haneul"}")
export DENOM="${DENOM:-uhaneul}"
export KEY_NAME="${KEY_NAME:-mykey}"

# Port configuration
export RPC_PORT="${RPC_PORT:-26657}"
export REST_PORT="${REST_PORT:-1317}"
export P2P_PORT="${P2P_PORT:-26656}"
export GRPC_PORT="${GRPC_PORT:-9090}"
export GRPC_WEB_PORT="${GRPC_WEB_PORT:-9091}"

# Block time configuration
export TIMEOUT_COMMIT="${TIMEOUT_COMMIT:-3s}"

echo "🚀 Haneul Local Blockchain Setup"
echo "=================================="
echo "Chain ID: $CHAIN_ID"
echo "Moniker: $MONIKER"
echo "Home Dir: $HOME_DIR"
echo "Denom: $DENOM"
echo ""

# Check if haneuld is installed
if ! command -v haneuld &> /dev/null; then
    echo "❌ haneuld not found. Please run 'make install' first."
    exit 1
fi

echo "✓ Found haneuld: $(which haneuld)"
haneuld version
echo ""

# Clean previous data if requested
if [ "$CLEAN" = "true" ]; then
    echo "🧹 Cleaning previous blockchain data..."
    rm -rf $HOME_DIR
    echo "✓ Cleaned $HOME_DIR"
    echo ""
fi

# Initialize if not already done
if [ ! -d "$HOME_DIR/config" ]; then
    echo "📝 Initializing new blockchain..."

    # Initialize node
    haneuld init $MONIKER --chain-id $CHAIN_ID --default-denom $DENOM --home $HOME_DIR
    echo "✓ Initialized node"

    # Set client configuration
    haneuld config set client chain-id $CHAIN_ID --home $HOME_DIR
    haneuld config set client keyring-backend $KEYRING --home $HOME_DIR
    echo "✓ Set client configuration"

    # Create keys
    echo "🔑 Creating validator key..."
    if ! haneuld keys show $KEY_NAME --keyring-backend $KEYRING --home $HOME_DIR &> /dev/null; then
        haneuld keys add $KEY_NAME --keyring-backend $KEYRING --home $HOME_DIR
        echo "✓ Created key: $KEY_NAME"
    else
        echo "✓ Key already exists: $KEY_NAME"
    fi

    # Get validator address
    VALIDATOR_ADDR=$(haneuld keys show $KEY_NAME -a --keyring-backend $KEYRING --home $HOME_DIR)
    echo "✓ Validator address: $VALIDATOR_ADDR"
    echo ""

    # Add genesis account
    echo "💰 Adding genesis account..."
    haneuld genesis add-genesis-account $KEY_NAME 100000000000${DENOM} --keyring-backend $KEYRING --home $HOME_DIR
    echo "✓ Added genesis account with 100000000000${DENOM}"

    # Create genesis transaction
    echo "📜 Creating genesis transaction..."
    haneuld genesis gentx $KEY_NAME 50000000000${DENOM} \
        --keyring-backend $KEYRING \
        --chain-id $CHAIN_ID \
        --home $HOME_DIR \
        --commission-rate="0.05" \
        --commission-max-rate="0.20" \
        --commission-max-change-rate="0.01" \
        --min-self-delegation="1"
    echo "✓ Created genesis transaction"

    # Collect genesis transactions
    haneuld genesis collect-gentxs --home $HOME_DIR
    echo "✓ Collected genesis transactions"

    # Validate genesis
    haneuld genesis validate-genesis --home $HOME_DIR
    echo "✓ Genesis file is valid"
    echo ""

    # Update configuration for faster blocks
    echo "⚙️  Updating configuration..."
    sed -i.bak "s/timeout_commit = \".*\"/timeout_commit = \"$TIMEOUT_COMMIT\"/" $HOME_DIR/config/config.toml
    sed -i.bak "s/cors_allowed_origins = \[\]/cors_allowed_origins = [\"*\"]/" $HOME_DIR/config/config.toml

    # Enable API
    sed -i.bak "s/enable = false/enable = true/" $HOME_DIR/config/app.toml
    sed -i.bak "s/swagger = false/swagger = true/" $HOME_DIR/config/app.toml
    echo "✓ Updated configuration"
    echo ""
else
    echo "✓ Blockchain already initialized at $HOME_DIR"
    echo ""
fi

echo "🎯 Starting Haneul node..."
echo "=================================="
echo "RPC endpoint: http://localhost:$RPC_PORT"
echo "REST endpoint: http://localhost:$REST_PORT"
echo "GRPC endpoint: localhost:$GRPC_PORT"
echo ""
echo "Press Ctrl+C to stop the node"
echo ""

# Start the node
haneuld start \
    --home $HOME_DIR \
    --rpc.laddr tcp://0.0.0.0:$RPC_PORT \
    --grpc.address 0.0.0.0:$GRPC_PORT \
    --p2p.laddr tcp://0.0.0.0:$P2P_PORT
