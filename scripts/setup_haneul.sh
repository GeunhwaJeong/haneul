#!/bin/bash
# Haneul Blockchain Setup Script
# This script updates all references to ensure proper Haneul configuration

set -e

echo "🔄 Starting Haneul blockchain setup..."
echo ""

# Color codes for output
GREEN='\033[0.32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to display progress
function log_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

function log_warn() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

function log_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Create backup
BACKUP_DIR=".haneul_setup_backup_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"
log_success "Created backup directory: $BACKUP_DIR"

# Backup files that will be modified
files_to_backup=(
    "scripts/test_node.sh"
    "scripts/hermes/transfer.sh"
    "scripts/hermes/cw20/helpers.sh"
    "scripts/hermes/cw20/run.sh"
    "x/cw-hooks/script.sh"
)

for file in "${files_to_backup[@]}"; do
    if [ -f "$file" ]; then
        cp "$file" "$BACKUP_DIR/"
        log_success "Backed up: $file"
    fi
done

echo ""
echo "📝 Updating configuration files..."
echo ""

# Update scripts/test_node.sh
if [ -f "scripts/test_node.sh" ]; then
    sed -i.bak 's|HOME_DIR="~/\.juno1"|HOME_DIR="~/.haneul1"|g' scripts/test_node.sh
    sed -i.bak 's|HOME_DIR="~/\.juno2"|HOME_DIR="~/.haneul2"|g' scripts/test_node.sh
    sed -i.bak 's|HOME_DIR:-"~/\.juno"|HOME_DIR:-"~/.haneul"|g' scripts/test_node.sh
    rm scripts/test_node.sh.bak 2>/dev/null || true
    log_success "Updated: scripts/test_node.sh"
fi

# Update scripts/hermes/transfer.sh
if [ -f "scripts/hermes/transfer.sh" ]; then
    sed -i.bak 's|JUNOD_NODE|HANEULD_NODE|g' scripts/hermes/transfer.sh
    sed -i.bak 's|\.juno1|.haneul1|g' scripts/hermes/transfer.sh
    sed -i.bak 's|--from juno1|--from haneul1|g' scripts/hermes/transfer.sh
    rm scripts/hermes/transfer.sh.bak 2>/dev/null || true
    log_success "Updated: scripts/hermes/transfer.sh"
fi

# Update scripts/hermes/cw20/helpers.sh
if [ -f "scripts/hermes/cw20/helpers.sh" ]; then
    sed -i.bak 's|JUNOD_COMMAND_ARGS|HANEULD_COMMAND_ARGS|g' scripts/hermes/cw20/helpers.sh
    rm scripts/hermes/cw20/helpers.sh.bak 2>/dev/null || true
    log_success "Updated: scripts/hermes/cw20/helpers.sh"
fi

# Update scripts/hermes/cw20/run.sh
if [ -f "scripts/hermes/cw20/run.sh" ]; then
    sed -i.bak 's|JUNOD_NODE|HANEULD_NODE|g' scripts/hermes/cw20/run.sh
    sed -i.bak 's|JUNOD_COMMAND_ARGS|HANEULD_COMMAND_ARGS|g' scripts/hermes/cw20/run.sh
    sed -i.bak 's|--from juno1|--from haneul1|g' scripts/hermes/cw20/run.sh
    sed -i.bak 's|junod tx|haneuld tx|g' scripts/hermes/cw20/run.sh
    rm scripts/hermes/cw20/run.sh.bak 2>/dev/null || true
    log_success "Updated: scripts/hermes/cw20/run.sh"
fi

# Update x/cw-hooks/script.sh
if [ -f "x/cw-hooks/script.sh" ]; then
    sed -i.bak 's|JUNOD_NODE|HANEULD_NODE|g' x/cw-hooks/script.sh
    sed -i.bak 's|--from=juno1|--from=haneul1|g' x/cw-hooks/script.sh
    sed -i.bak 's|--from=juno2|--from=haneul2|g' x/cw-hooks/script.sh
    sed -i.bak 's|\.juno1|.haneul1|g' x/cw-hooks/script.sh
    sed -i.bak 's|\.juno"|.haneul"|g' x/cw-hooks/script.sh
    sed -i.bak 's|junod tx|haneuld tx|g' x/cw-hooks/script.sh
    sed -i.bak 's|junod tendermint|haneuld tendermint|g' x/cw-hooks/script.sh
    sed -i.bak 's|junod export|haneuld export|g' x/cw-hooks/script.sh
    rm x/cw-hooks/script.sh.bak 2>/dev/null || true
    log_success "Updated: x/cw-hooks/script.sh"
fi

echo ""
echo "📋 Summary of changes:"
echo ""
echo "✓ Updated environment variables: JUNOD_* → HANEULD_*"
echo "✓ Updated directory paths: ~/.juno → ~/.haneul"
echo "✓ Updated key names: juno1/juno2 → haneul1/haneul2"
echo "✓ Updated binary references: junod → haneuld"
echo ""
echo "📦 Backup created at: $BACKUP_DIR"
echo ""
echo "⚠  Note: Test files keep descriptive variable names (this is normal)"
echo "⚠  Note: Historical addresses in upgrade handlers are preserved"
echo ""
log_success "Haneul setup complete! 🎉"
echo ""
echo "Next steps:"
echo "  1. Run: make install"
echo "  2. Test: haneuld version"
echo "  3. Initialize: rm -rf ~/.haneul && haneuld init mynode --chain-id local-1 --default-denom uhaneul"
echo ""
