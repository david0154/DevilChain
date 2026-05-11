#!/usr/bin/env bash
# DevilChain Testnet Genesis Script
# Creates genesis block, initializes validator set, seeds balances

set -euo pipefail

NETWORK="devilchain-testnet"
CHAIN_ID="devl-testnet-1"
GENESIS_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
DB_PATH="/data/devilchain"
CONFIG_DIR="/etc/devilchain"
BIN="devilchain-node"

echo "======================================"
echo "  DevilChain Network Testnet Genesis"
echo "  Network : $NETWORK"
echo "  Chain ID: $CHAIN_ID"
echo "  Time    : $GENESIS_TIME"
echo "======================================"

# 1. Create directories
echo "[1/7] Creating directories..."
mkdir -p "$DB_PATH" "$CONFIG_DIR"

# 2. Check binary
echo "[2/7] Checking node binary..."
if ! command -v "$BIN" &>/dev/null; then
  echo "  Binary not found in PATH. Building from source..."
  cd /opt/devilchain/core && cargo build --release
  cp target/release/devilchain-node /usr/local/bin/
fi
echo "  Binary: $(which $BIN)"

# 3. Initialize node config
echo "[3/7] Initializing node configuration..."
$BIN init --type validator

# 4. Write genesis.json
echo "[4/7] Writing genesis block..."
cat > "$CONFIG_DIR/genesis.json" <<GENESIS
{
  "network": "$NETWORK",
  "chain_id": "$CHAIN_ID",
  "genesis_time": "$GENESIS_TIME",
  "native_coin": "DevilCoin",
  "symbol": "DVL",
  "decimals": 18,
  "consensus": "DHP",
  "block_time_secs": 3,
  "block_gas_limit": 10000000,
  "min_validator_stake": "100000000000000000000",
  "initial_validators": [
    {
      "address": "db1xval_genesis_001",
      "staked": "10000000000000000000000",
      "active": true
    }
  ],
  "genesis_balances": {
    "db1xdao_treasury": "150000000000000000000000000",
    "db1xecosystem": "200000000000000000000000000",
    "db1xteam": "100000000000000000000000000",
    "db1xmining_pool": "350000000000000000000000000",
    "db1xcommunity": "50000000000000000000000000",
    "db1xvalidator_pool": "100000000000000000000000000",
    "db1xinvestors": "50000000000000000000000000"
  },
  "tokenomics": {
    "total_supply": "1000000000000000000000000000",
    "mining_rewards_pct": 35,
    "ecosystem_pct": 20,
    "dao_treasury_pct": 15,
    "team_pct": 10,
    "validators_pct": 10,
    "investors_pct": 5,
    "community_pct": 5
  }
}
GENESIS
echo "  Genesis written to $CONFIG_DIR/genesis.json"

# 5. Write node config
echo "[5/7] Writing node config..."
cat > "$CONFIG_DIR/config.toml" <<CONFIG
[node]
network = "$NETWORK"
chain_id = "$CHAIN_ID"
mode = "validator"
db_path = "$DB_PATH"
log_level = "info"

[api]
rest_port = 8545
graphql_port = 8546
ai_port = 8547
storage_port = 8548

[consensus]
block_time = 3
min_stake = 100
dhp_version = "1.0"

[mining]
algorithm = "DVLHash-AI"
threads = 4
anti_asic = true

[p2p]
port = 30303
max_peers = 50

[ai]
enabled = true
threshold = 0.75
onnx_model = "/etc/devilchain/models/devilguard.onnx"
CONFIG
echo "  Config written to $CONFIG_DIR/config.toml"

# 6. Create systemd service
echo "[6/7] Creating systemd service..."
cat > /etc/systemd/system/devilchain.service <<SERVICE
[Unit]
Description=DevilChain Network Node
After=network.target

[Service]
Type=simple
Restart=always
RestartSec=5
User=root
ExecStart=/usr/local/bin/devilchain-node start --mode validator --db-path $DB_PATH
StandardOutput=journal
StandardError=journal
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
SERVICE
systemctl daemon-reload
systemctl enable devilchain
echo "  Systemd service created & enabled"

# 7. Start testnet
echo "[7/7] Starting DevilChain testnet..."
systemctl start devilchain
sleep 3
systemctl is-active --quiet devilchain && echo "  ✅ Node running!" || echo "  ⚠️  Node failed to start. Check: journalctl -u devilchain -f"

echo ""
echo "======================================"
echo "  DevilChain Testnet LIVE"
echo "  REST API : http://0.0.0.0:8545"
echo "  GraphQL  : http://0.0.0.0:8546/graphql"
echo "  AI API   : http://0.0.0.0:8547"
echo "  Storage  : http://0.0.0.0:8548"
echo "  Logs     : journalctl -u devilchain -f"
echo "======================================"
