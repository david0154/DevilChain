#!/bin/bash
# DevilChain Node Setup Script
# Supports: Ubuntu 22.04 LTS

set -e

echo "=================================="
echo " DevilChain Network Node Setup"
echo "=================================="

# 1. Update system
apt update && apt upgrade -y
apt install -y curl git build-essential pkg-config libssl-dev clang

# 2. Install Rust
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi
echo "Rust version: $(rustc --version)"

# 3. Install Go (for secondary services)
if ! command -v go &> /dev/null; then
    wget https://go.dev/dl/go1.22.0.linux-amd64.tar.gz
    tar -C /usr/local -xzf go1.22.0.linux-amd64.tar.gz
    export PATH=$PATH:/usr/local/go/bin
fi
echo "Go version: $(go version)"

# 4. Clone DevilChain
git clone https://github.com/david0154/DevilChain.git /opt/devilchain
cd /opt/devilchain/core

# 5. Build
cargo build --release
cp target/release/devilchain-node /usr/local/bin/devilchain-node

echo ""
echo "[OK] DevilChain node installed at /usr/local/bin/devilchain-node"
echo "Run: devilchain-node init --type=lite"
echo "Run: devilchain-node start"
echo "API: http://localhost:8545"
