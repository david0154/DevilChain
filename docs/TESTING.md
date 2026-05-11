# DevilChain Network — Complete Testing Guide

> **Developed by [Nexuzy Lab](https://nexuzy.tech) | Powered by [Devil One](https://devilone.in)**

---

## Overview

DevilChain ships with a complete, automated testing pipeline covering:

- ✅ Blockchain node (REST + GraphQL)
- ✅ Transaction sending & gas fees
- ✅ Wallet generation & lifecycle
- ✅ Coin (DVC/DVL) minting & tokenomics
- ✅ AI fraud/scam scanning (DevilGuard)
- ✅ Decentralized storage (DevilStorage)
- ✅ Mining block production (DVLHash-AI)
- ✅ DAO governance votes
- ✅ Staking & unstaking
- ✅ Bridge health
- ✅ Explorer web pages

---

## Test Environment Setup

### Option A — Docker (Recommended, zero config)

```bash
git clone https://github.com/david0154/DevilChain.git
cd DevilChain
bash docker/start.sh all    # Start all 12 services
bash docker/start.sh test   # Run full pytest suite
```

### Option B — Local (node running manually)

```bash
# Start node locally first
cd core && cargo run -- start --mode validator

# In another terminal, run quick test
bash docker/quick_test.sh
```

### Option C — Python pytest only

```bash
pip install requests pytest httpx colorama cryptography
export NODE_API=http://localhost:8545
export AI_API=http://localhost:8547
export STORAGE_API=http://localhost:8548

cd docker/tests
pytest . -v --tb=short --color=yes
```

---

## Test Files

| File | Tests | What it covers |
|---|---|---|
| `docker/tests/test_blockchain.py` | 22 tests | Node, TXs, fees, GraphQL, AI scan, storage, mining, explorer, bridge |
| `docker/tests/test_wallet.py` | 13 tests | Wallet generation, Ed25519 signing, full TX lifecycle, DAO vote, staking |
| `docker/tests/generate_wallets.py` | CLI tool | Generate `db1x...` wallets with Ed25519 keypairs |
| `docker/tests/generate_coin.py` | CLI tool | DVC/DVL tokenomics, mint simulation, faucet, balance check |
| `docker/quick_test.sh` | Shell script | All APIs via `curl` with generated wallets |

---

## Wallet Generation

All test wallets use real **Ed25519** cryptography and the `db1x...` address format:

```
Address = "db1x" + SHA256(SHA256(pubkey))[:32 hex chars]
```

Transaction signatures use Ed25519 (via Python `cryptography` lib).

```bash
# Generate 3 wallets
python docker/tests/generate_wallets.py

# Generate + export to JSON
python docker/tests/generate_wallets.py --count 5 --export

# Faucet curl commands are printed automatically for each wallet
```

---

## Coin (DVC/DVL) Verification

```bash
# Full tokenomics display + wallet generation
python docker/tests/generate_coin.py

# Simulate mining reward (50 DVC + AI bonus)
python docker/tests/generate_coin.py --mint

# Check live balance
python docker/tests/generate_coin.py --balance db1x3f8a2c...

# Fund via faucet
python docker/tests/generate_coin.py --faucet db1x3f8a2c... --amount 1000
```

**Tokenomics verified in tests:**
- Total supply: 1,000,000,000 DVC
- Block reward: 50 DVC per block
- AI mining bonus: 0.00–5.00 DVC extra
- Gas fee: 0.01 DVC per TX
- Min validator stake: 100 DVC

---

## AI Scan Tests (DevilGuard)

| Scenario | Input | Expected Result |
|---|---|---|
| Safe TX | Normal amount, real addresses | `verdict: SAFE`, score > 0.75 |
| Scam TX | 999,999 DVC, `db1xscam` addr | `verdict: FLAGGED`, score < 0.75 |
| Clean contract | Standard ERC-20 transfer | Low risk |
| Rug pull contract | `selfdestruct`, `withdrawAll onlyOwner` | `HIGH_RISK` |
| Clean text | Normal post | `safe: true` |
| Spam text | "FREE AIRDROP! Guaranteed profit!" | `safe: false` |
| Fake node | 0 stake + claims validator | `is_fake: true` |
| Blacklist | Any address | Added to blacklist |

---

## Transaction Flow Tested

```
Generate Alice wallet (Ed25519)
         ↓
Generate Bob wallet (Ed25519)
         ↓
Faucet → fund Alice (1000 DVC)
         ↓
Alice signs TX (Ed25519 signature)
         ↓
POST /api/send  { from, to, amount: 50, gas_fee: 0.01, signature }
         ↓
Node validates TX + deducts fee
         ↓
AI scans TX (DevilGuard)
         ↓
TX confirmed in block
         ↓
GET /api/tx/{hash}  → verify from/to/amount
         ↓
GET /api/wallet/bob  → verify balance increased
```

---

## Expected Test Output

```
===================== test session starts ======================

=== GENERATING TEST WALLETS ===
  [Alice]  db1x3f8a2c1d4e9b7f...
  [Bob]    db1x9b2e1c7d4f3a8e...
  [Miner]  db1xc4d8f2a1e7b3c9...

✅  PASS  Wallet format: db1x + 32 hex chars
✅  PASS  All 3 addresses unique
✅  PASS  Faucet: 1000 DVC → Alice
✅  PASS  Alice balance: 1000 DVC
✅  PASS  Alice → Bob 50 DVC (fee: 0.01 DVC)
✅  PASS  TX verified: from=db1x... to=db1x... amount=50
✅  PASS  Bob balance: 50 DVC
✅  PASS  Alice staked 100 DVC
✅  PASS  Alice unstaked 50 DVC
✅  PASS  DAO vote submitted
✅  PASS  AI scan: score=0.97 verdict=SAFE

=== WALLET TEST SUMMARY ===
  Alice    db1x3f8a...  balance=950 DVC
  Bob      db1x9b2e...  balance=50 DVC
  Miner    db1xc4d8...  balance=0 DVC

23 passed in 14.3s
```

---

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `NODE_API` | `http://localhost:8545` | Blockchain node REST URL |
| `GRAPHQL_API` | `http://localhost:8546` | GraphQL endpoint |
| `AI_API` | `http://localhost:8547` | DevilGuard AI URL |
| `STORAGE_API` | `http://localhost:8548` | DevilStorage URL |
| `BRIDGE_API` | `http://localhost:8549` | DevilBridge URL |

---

<p align="center">
  DevilChain Testing Guide &nbsp;|&nbsp;
  <a href="https://nexuzy.tech">Nexuzy Lab</a> &nbsp;|&nbsp;
  <a href="https://devilone.in">Devil One</a>
</p>
