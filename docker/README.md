# 🐳 DevilChain Docker Testing Stack

> **Developed by [Nexuzy Lab](https://nexuzy.tech) | Powered by [Devil One](https://devilone.in)**  
> Lead Developer: [David @david0154](https://github.com/david0154)

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Service Map](#service-map)
4. [Test Commands](#test-commands)
5. [Wallet & Coin Generation](#wallet--coin-generation)
6. [API Testing Guide](#api-testing-guide)
7. [Test Coverage](#test-coverage)
8. [Troubleshooting](#troubleshooting)

---

## Prerequisites

| Tool | Version | Install |
|---|---|---|
| Docker | 24+ | [docker.com](https://docs.docker.com/get-docker/) |
| Docker Compose | v2+ | Included with Docker Desktop |
| Python | 3.11+ | For standalone tools |
| curl | Any | Pre-installed on Linux/macOS |
| bash | Any | Pre-installed on Linux/macOS |

> **Windows users:** Use WSL2 or Git Bash.

---

## Quick Start

### 1. Clone & Start Everything
```bash
git clone https://github.com/david0154/DevilChain.git
cd DevilChain

# Start full stack (builds all images)
bash docker/start.sh all
```

### 2. Run Full Test Suite
```bash
bash docker/start.sh test
```

### 3. Quick Shell Test (no Docker needed, node must be running)
```bash
bash docker/quick_test.sh
```

---

## Service Map

| Container | Port | Description |
|---|---|---|
| `devilchain-node` | **8545** | Blockchain REST API |
| `devilchain-node` | **8546** | GraphQL API |
| `devilguard-ai` | **8547** | AI Scan / DevilGuard |
| `devilstorage` | **8548** | Decentralized Storage Node |
| `devilbridge` | **8549** | Cross-chain Bridge Relayer |
| `devilscan` | **3000** | Explorer Web UI |
| `devilsocial` | **3001** | DevilSocial Web App |
| `devilchat` | **3002** | DevilChat Web App |
| `postgres` | **5432** | PostgreSQL (wallets, blocks, fees) |
| `redis` | **6379** | Redis cache |
| `mongodb` | **27017** | MongoDB (AI logs, social) |
| `devilmine` | — | Mining Worker (DVLHash-AI) |
| `devil-tester` | — | Automated pytest runner |

---

## Test Commands

### `start.sh` — Master Control Script

```bash
# Start full stack + wait for health
bash docker/start.sh all

# Start node + databases only
bash docker/start.sh node

# Run full automated pytest suite
bash docker/start.sh test

# Show live status + health check
bash docker/start.sh status

# Tail logs (node + AI)
bash docker/start.sh logs

# Stop all services
bash docker/start.sh stop

# Remove everything (containers + volumes + images)
bash docker/start.sh clean
```

### `quick_test.sh` — Shell Test (no pytest needed)

```bash
bash docker/quick_test.sh
```

Tests every API with `curl` and prints ✅/❌ per endpoint. Also:
- Generates fresh `db1x...` wallet addresses
- Funds wallets via faucet
- Sends real test transactions
- Checks mining block progress

---

## Wallet & Coin Generation

### Generate Wallets
```bash
# Generate 3 wallets (Alice, Bob, Miner)
python docker/tests/generate_wallets.py

# Generate 5 wallets with custom labels
python docker/tests/generate_wallets.py --count 5 --labels alice bob miner validator dao

# Export to wallets.json
python docker/tests/generate_wallets.py --count 3 --export
```

**Sample Output:**
```
┌─────────────────────────────────────────────────────┐
│  DevilChain Wallet #1  [alice]
├─────────────────────────────────────────────────────┤
│  Address    : db1x3f8a2c1d4e9b7f2a3e8c1d4b9f2a7e8c
│  Public Key : 7a3f9c12e4b8d2f1a6e3c9d7b4f2...
│  Mnemonic   : devil chain vault stake hash node...
└─────────────────────────────────────────────────────┘
```

### Generate Coins (DVC/DVL)
```bash
# Show full tokenomics + generate wallets
python docker/tests/generate_coin.py

# Simulate mining block rewards
python docker/tests/generate_coin.py --mint

# Check live balance from node
python docker/tests/generate_coin.py --balance db1x<address>

# Drip 1000 DVC from testnet faucet
python docker/tests/generate_coin.py --faucet db1x<address> --amount 1000

# Export tokenomics + wallets
python docker/tests/generate_coin.py --mint --export
```

---

## API Testing Guide

### Blockchain Node (port 8545)

```bash
# Node status
curl http://localhost:8545/api/status

# Latest block
curl http://localhost:8545/api/block/latest

# Block by height
curl http://localhost:8545/api/block/1

# Transaction by hash
curl http://localhost:8545/api/tx/<tx_hash>

# Wallet balance
curl http://localhost:8545/api/wallet/db1x<address>

# Validator list
curl http://localhost:8545/api/validators

# DAO proposals
curl http://localhost:8545/api/dao/proposals

# Coin info
curl http://localhost:8545/api/coin

# Send transaction
curl -X POST http://localhost:8545/api/send \
  -H 'Content-Type: application/json' \
  -d '{"from":"db1x...","to":"db1x...","amount":50,"gas_fee":0.01,"signature":"ed25519_sig"}'

# Stake DVC
curl -X POST http://localhost:8545/api/stake \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1x...","amount":100,"signature":"ed25519_sig"}'

# Unstake DVC
curl -X POST http://localhost:8545/api/unstake \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1x...","amount":50,"signature":"ed25519_sig"}'

# DAO vote
curl -X POST http://localhost:8545/api/vote \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1x...","proposal_id":1,"vote":true,"signature":"ed25519_sig"}'

# Testnet faucet
curl -X POST http://localhost:8545/api/faucet \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1x...","amount":1000}'
```

### GraphQL API (port 8546)

```bash
# Network status
curl -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ status { network coin symbol latestHeight } }"}'

# Latest block
curl -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ latestBlock { height hash txCount aiScore } }"}'

# Validators
curl -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ validators { address stakedDvc reputationScore active } }"}'
```

### DevilGuard AI (port 8547)

```bash
# Health check
curl http://localhost:8547/health

# Scan a safe transaction
curl -X POST http://localhost:8547/scan/tx \
  -H 'Content-Type: application/json' \
  -d '{"from_addr":"db1xalice","to_addr":"db1xbob","amount":10,"gas_fee":0.01}'

# Scan a scam transaction (should return FLAGGED)
curl -X POST http://localhost:8547/scan/tx \
  -H 'Content-Type: application/json' \
  -d '{"from_addr":"db1xscam","to_addr":"db1xvictim","amount":999999,"gas_fee":0.0001}'

# Scan smart contract source (rug pull detection)
curl -X POST http://localhost:8547/scan/contract \
  -H 'Content-Type: application/json' \
  -d '{"source_code":"function withdrawAll() onlyOwner { selfdestruct(owner); }","name":"RugToken"}'

# NLP spam moderation
curl -X POST http://localhost:8547/moderate \
  -H 'Content-Type: application/json' \
  -d '{"text":"FREE AIRDROP! Guaranteed 10x profit! Send now!"}'

# Fake node detection
curl -X POST http://localhost:8547/detect/fake-node \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1xfake","staked":0,"is_validator":true,"uptime_percent":2}'

# Blacklist address
curl -X POST http://localhost:8547/blacklist \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1xscammer"}'
```

### DevilStorage (port 8548)

```bash
# Storage stats
curl http://localhost:8548/stats

# Store a file (base64 encoded)
FILE_B64=$(echo -n 'Hello DevilChain 2026' | base64)
curl -X POST http://localhost:8548/store \
  -H 'Content-Type: application/json' \
  -d "{\"data_b64\":\"$FILE_B64\",\"file_name\":\"hello.txt\",\"owner\":\"db1xalice\",\"is_public\":true}"

# Retrieve file by CID
curl http://localhost:8548/retrieve/<cid>
```

---

## Test Coverage

### `test_blockchain.py` — Core Node Tests
| Test | Description |
|---|---|
| `test_node_status` | Node reachable, returns network info |
| `test_latest_block` | Latest block height accessible |
| `test_block_by_height` | Block #0 (genesis) accessible |
| `test_validators_list` | Validator list returned |
| `test_dao_proposals` | DAO proposals accessible |
| `test_wallet_balance` | Wallet balance query works |

### `test_blockchain.py` — Transaction & Fee Tests
| Test | Description |
|---|---|
| `test_send_transaction` | Send DVC with gas fee |
| `test_fee_calculation` | Gas fee correctly included |
| `test_staking` | Stake DVC to validator pool |
| `test_unstaking` | Unstake DVC from pool |

### `test_blockchain.py` — GraphQL Tests
| Test | Description |
|---|---|
| `test_graphql_status` | Status query returns network info |
| `test_graphql_latest_block` | latestBlock query works |
| `test_graphql_validators` | Validators query works |

### `test_blockchain.py` — AI Scan Tests
| Test | Description |
|---|---|
| `test_ai_health` | AI service running |
| `test_scan_safe_transaction` | Safe TX → score > 0.75, SAFE |
| `test_scan_suspicious_transaction` | Scam TX → FLAGGED |
| `test_scan_safe_contract` | Clean contract → passes |
| `test_scan_rug_contract` | Rug pull → HIGH_RISK |
| `test_nlp_moderation_clean` | Clean text → safe=true |
| `test_nlp_moderation_spam` | Spam text → safe=false |
| `test_detect_fake_node` | Fake validator → is_fake=true |
| `test_blacklist_address` | Address blacklisted |

### `test_blockchain.py` — Storage Tests
| Test | Description |
|---|---|
| `test_storage_stats` | Stats endpoint works |
| `test_store_and_retrieve_file` | Store file → get CID → retrieve → verify bytes |

### `test_blockchain.py` — Mining Tests
| Test | Description |
|---|---|
| `test_mining_produces_blocks` | Block height increases over time |
| `test_mining_reward_address` | Miner wallet exists |

### `test_wallet.py` — Wallet & TX Lifecycle
| Test | Description |
|---|---|
| `test_01_wallet_address_format` | `db1x...` format + 36-char length |
| `test_02_wallet_uniqueness` | All addresses unique |
| `test_03_faucet_fund_alice` | Fund Alice 1000 DVC |
| `test_04_check_alice_balance` | Check Alice balance on-chain |
| `test_05_alice_sends_to_bob` | Alice → Bob 50 DVC + 0.01 gas (Ed25519 signed) |
| `test_06_verify_transaction` | Look up TX by hash |
| `test_07_bob_balance_increased` | Bob's balance reflects received DVC |
| `test_08_alice_stakes_dvc` | Alice stakes 100 DVC |
| `test_09_alice_unstakes_dvc` | Alice unstakes 50 DVC |
| `test_10_dao_vote` | Alice votes on DAO proposal #1 |
| `test_11_miner_wallet_rewards` | Miner wallet reward check |
| `test_12_ai_scan_generated_tx` | AI scans real wallet TX → SAFE |
| `test_13_print_wallet_summary` | Final balance summary |

---

## Troubleshooting

### Node not starting
```bash
# Check logs
docker compose -f docker/docker-compose.yml logs devilchain-node

# Rebuild core
docker compose -f docker/docker-compose.yml build --no-cache devilchain-node
```

### AI service not responding
```bash
docker compose -f docker/docker-compose.yml logs devilguard-ai
# Check Python deps installed
docker exec devilguard_ai pip list
```

### Explorer blank / not loading
```bash
docker compose -f docker/docker-compose.yml logs devilscan
# Make sure NEXT_PUBLIC_API_URL is set
docker exec devilscan env | grep API
```

### Port already in use
```bash
# Find what's using port 8545
lsof -i :8545
# or
ss -tlnp | grep 8545
```

### Full reset
```bash
bash docker/start.sh clean
bash docker/start.sh all
```

---

<p align="center">
  <b>DevilChain Network Docker Docs</b><br/>
  <a href="https://nexuzy.tech">Nexuzy Lab</a> &nbsp;|&nbsp;
  <a href="https://devilone.in">Devil One</a> &nbsp;|&nbsp;
  <a href="https://github.com/david0154/DevilChain">GitHub</a>
</p>
