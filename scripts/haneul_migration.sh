#!/bin/bash
# Haneul Blockchain Update Script
# This script updates all references to establish Haneul as an independent blockchain

set -e

echo "🔄 Starting Haneul blockchain updates..."
echo ""

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

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
BACKUP_DIR=".haneul_migration_backup_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"
log_success "Created backup directory: $BACKUP_DIR"

# Step 1: Rename proto directory structure
echo ""
echo "📁 Step 1: Updating proto directory structure..."
if [ -d "proto/juno" ]; then
    cp -r proto/juno "$BACKUP_DIR/"
    mv proto/juno proto/haneul
    log_success "Updated proto directory to haneul"
fi

# Step 2: Update proto package names in all .proto files
echo ""
echo "📝 Step 2: Updating proto package declarations..."
find proto/haneul -name "*.proto" -type f -exec sed -i.bak 's/package juno\./package haneul./g' {} \;
find proto/haneul -name "*.proto" -type f -exec sed -i.bak 's|option go_package = "github.com/GeunhwaJeong/haneul/api/juno/|option go_package = "github.com/GeunhwaJeong/haneul/api/haneul/|g' {} \;
find proto/haneul -name "*.proto" -type f -exec sed -i.bak 's|import "juno/|import "haneul/|g' {} \;
find proto/haneul -name "*.proto.bak" -delete
log_success "Updated proto package names"

# Step 3: Update Go imports in source files
echo ""
echo "📦 Step 3: Updating Go imports..."
find . -name "*.go" -type f ! -path "./api/*" -exec sed -i.bak 's|github.com/GeunhwaJeong/haneul/api/juno/|github.com/GeunhwaJeong/haneul/api/haneul/|g' {} \;
find . -name "*.go.bak" -delete
log_success "Updated Go imports"

# Step 4: Update test files variable names (preserve logic)
echo ""
echo "🧪 Step 4: Updating test files..."
find ./interchaintest -name "*.go" -type f -exec sed -i.bak 's/\bjuno\b := chains/haneul := chains/g' {} \;
find ./interchaintest -name "*.go" -type f -exec sed -i.bak 's/\bjuno\./haneul./g' {} \;
find ./interchaintest -name "*.go" -type f -exec sed -i.bak 's/, juno\b/, haneul/g' {} \;
find ./interchaintest -name "*.go" -type f -exec sed -i.bak 's/(juno\b/(haneul/g' {} \;
find ./interchaintest -name "*.go" -type f -exec sed -i.bak 's/Name:.*"juno"/Name:          "haneul"/g' {} \;
find ./interchaintest -name "*.go" -type f -exec sed -i.bak 's/repo = "juno"/repo = "haneul"/g' {} \;
find ./interchaintest -name "*.go.bak" -delete
log_success "Updated test files"

# Step 5: Update documentation
echo ""
echo "📚 Step 5: Updating documentation..."
find . -name "*.md" -type f ! -path "./$BACKUP_DIR/*" ! -path "./.haneul_setup_backup*/*" ! -path "./.haneul_migration_backup*/*" -exec sed -i.bak 's/Juno Network/Haneul Network/g' {} \;
find . -name "*.md" -type f ! -path "./$BACKUP_DIR/*" ! -path "./.haneul_setup_backup*/*" ! -path "./.haneul_migration_backup*/*" -exec sed -i.bak 's/juno_/haneul_/g' {} \;
find . -name "*.md" -type f ! -path "./$BACKUP_DIR/*" ! -path "./.haneul_setup_backup*/*" ! -path "./.haneul_migration_backup*/*" -exec sed -i.bak 's|CosmosContracts/juno|GeunhwaJeong/haneul|g' {} \;
find . -name "*.md" -type f ! -path "./$BACKUP_DIR/*" ! -path "./.haneul_setup_backup*/*" ! -path "./.haneul_migration_backup*/*" -exec sed -i.bak 's/\bjuno1\b/haneul1/g' {} \;
find . -name "*.md.bak" -delete
log_success "Updated documentation files"

# Step 6: Update Docker references
echo ""
echo "🐳 Step 6: Updating Docker references..."
if [ -f "Dockerfile" ]; then
    sed -i.bak 's|cosmoscontracts/haneul|geunhwajeong/haneul|g' Dockerfile
    rm -f Dockerfile.bak
    log_success "Updated Dockerfile"
fi

# Step 7: Update buf.yaml proto config
echo ""
echo "⚙️  Step 7: Updating proto configuration..."
if [ -f "proto/buf.yaml" ]; then
    sed -i.bak 's|buf.build/CosmosContracts/haneul|buf.build/GeunhwaJeong/haneul|g' proto/buf.yaml
    rm -f proto/buf.yaml.bak
    log_success "Updated proto/buf.yaml"
fi

# Step 8: Update GitHub templates
echo ""
echo "🔧 Step 8: Updating GitHub templates..."
find .github -name "*.md" -type f -exec sed -i.bak 's/Juno/Haneul/g' {} \;
find .github -name "*.md" -type f -exec sed -i.bak 's/juno/haneul/g' {} \;
find .github -name "*.md.bak" -delete
log_success "Updated GitHub templates"

# Step 9: Update app/keepers if it has juno module imports
echo ""
echo "🔑 Step 9: Checking keeper imports..."
if grep -q "juno\." app/keepers/keepers.go 2>/dev/null; then
    sed -i.bak 's/juno\./haneul./g' app/keepers/keepers.go
    rm -f app/keepers/keepers.go.bak
    log_success "Updated keeper imports"
else
    log_warn "No keeper imports to update"
fi

# Step 10: Rename api/juno directory to api/haneul
echo ""
echo "📁 Step 10: Updating generated API directory..."
if [ -d "api/juno" ]; then
    mv api/juno api/haneul
    log_success "Updated API directory to haneul"
fi

# Step 11: Update generated API files
echo ""
echo "🔄 Step 11: Updating generated API package names..."
find api/haneul -name "*.go" -type f -exec sed -i.bak 's|package juno|package haneul|g' {} \;
find api/haneul -name "*.go" -type f -exec sed -i.bak 's|github.com/GeunhwaJeong/haneul/api/juno/|github.com/GeunhwaJeong/haneul/api/haneul/|g' {} \;
find api/haneul -name "*.go.bak" -delete
log_success "Updated generated API files"

echo ""
echo "📋 Summary of changes:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Updated proto/juno → proto/haneul"
echo "✓ Updated proto package declarations"
echo "✓ Updated Go imports from api/juno to api/haneul"
echo "✓ Updated test variable names"
echo "✓ Updated documentation"
echo "✓ Updated Docker references"
echo "✓ Updated api/juno → api/haneul"
echo "✓ Updated generated API package names"
echo ""
echo "📦 Backup created at: $BACKUP_DIR"
echo ""
log_warn "IMPORTANT NEXT STEPS:"
echo "  1. Regenerate protobuf files: make proto-gen"
echo "  2. Run: go mod tidy"
echo "  3. Test build: make install"
echo "  4. Test local chain: ./scripts/local_haneul.sh"
echo ""
log_success "Haneul blockchain updates complete! 🎉"
