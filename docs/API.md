# DevilChain Network — API Reference

> **Developed by [Nexuzy Lab](https://nexuzy.tech) | Powered by [Devil One](https://devilone.in)**

---

## Base URLs

| API | URL | Protocol |
|---|---|---|
| REST Node | `http://localhost:8545` | HTTP/JSON |
| GraphQL | `http://localhost:8546/graphql` | HTTP/JSON |
| DevilGuard AI | `http://localhost:8547` | HTTP/JSON |
| DevilStorage | `http://localhost:8548` | HTTP/JSON |
| DevilBridge | `http://localhost:8549` | HTTP/JSON |

---

## REST API (Port 8545)

### GET `/api/status`
Returns network status.
```json
{
  "network": "devilchain-testnet",
  "chain_id": "devl-testnet-1",
  "coin": "DevilCoin",
  "symbol": "DVC",
  "chain_length": 1042,
  "total_txs": 588,
  "validator_count": 3
}
```

### GET `/api/block/latest`
```json
{
  "block_height": 1042,
  "block_hash": "dvl_abc123...",
  "previous_hash": "dvl_xyz...",
  "validator": "db1xval_001",
  "tx_count": 5,
  "nonce": 10042,
  "ai_score": 0.97,
  "timestamp": 1770000042
}
```

### GET `/api/block/{height}`
Fetch block by height number.

### GET `/api/tx/{hash}`
```json
{
  "tx_hash": "dvl_tx_abc123",
  "from": "db1xalice...",
  "to": "db1xbob...",
  "amount": 50.0,
  "gas_fee": 0.01,
  "timestamp": 1770000001,
  "ai_score": 0.97,
  "status": "confirmed"
}
```

### GET `/api/wallet/{address}`
```json
{
  "address": "db1xalice...",
  "balance": 950.0,
  "staked": 50.0,
  "tx_count": 3,
  "ai_risk_score": "Low"
}
```

### GET `/api/validators`
```json
{
  "validators": [
    {
      "address": "db1xval_001",
      "staked_dvc": 10000.0,
      "reputation_score": 98.5,
      "blocks_validated": 1042,
      "active": true
    }
  ]
}
```

### GET `/api/coin`
```json
{
  "name": "DevilCoin",
  "symbol": "DVC",
  "ticker": "DVL",
  "total_supply": 1000000000,
  "decimals": 18,
  "block_reward": 50.0,
  "min_gas_fee": 0.01,
  "min_stake": 100.0
}
```

### POST `/api/send`
```json
// Request
{
  "from": "db1xalice...",
  "to": "db1xbob...",
  "amount": 50.0,
  "gas_fee": 0.01,
  "signature": "ed25519_hex_signature"
}
// Response 200
{ "tx_hash": "dvl_tx_abc123", "status": "broadcast" }
```

### POST `/api/stake`
```json
{ "address": "db1x...", "amount": 100.0, "signature": "ed25519_sig" }
```

### POST `/api/unstake`
```json
{ "address": "db1x...", "amount": 50.0, "signature": "ed25519_sig" }
```

### POST `/api/vote`
```json
{ "address": "db1x...", "proposal_id": 1, "vote": true, "signature": "ed25519_sig" }
```

### POST `/api/faucet` *(testnet only)*
```json
{ "address": "db1x...", "amount": 1000.0 }
```

---

## GraphQL API (Port 8546)

**Endpoint:** `POST http://localhost:8546/graphql`

### Queries

```graphql
# Network status
query { status { network coin symbol latestHeight totalTxs } }

# Latest block
query { latestBlock { height hash previousHash txCount aiScore timestamp } }

# Block by height
query { block(height: 100) { height hash validator txCount } }

# Validators
query { validators { address stakedDvc reputationScore blocksValidated active } }

# Transaction
query { transaction(hash: "dvl_tx_abc") { txHash from to amount gasFee status } }

# Wallet
query { wallet(address: "db1x...") { address balance staked txCount } }
```

---

## DevilGuard AI API (Port 8547)

### GET `/health`
```json
{ "status": "ok", "model": "DevilGuard-v1", "threshold": 0.75 }
```

### POST `/scan/tx`
```json
// Request
{ "from_addr": "db1x...", "to_addr": "db1x...", "amount": 50.0, "gas_fee": 0.01 }

// Response
{
  "ai_score": 0.97,
  "verdict": "SAFE",         // SAFE | FLAGGED
  "risk_factors": [],
  "recommendation": "approve"
}
```

### POST `/scan/contract`
```json
// Request
{ "source_code": "pragma solidity...", "name": "MyToken" }

// Response
{
  "combined_risk_score": 0.12,
  "verdict": "SAFE",          // SAFE | MEDIUM_RISK | HIGH_RISK
  "rug_pull_risk": 0.05,
  "overflow_risk": 0.07,
  "graph_warnings": []
}
```

### POST `/moderate`
```json
// Request
{ "text": "Hello world" }

// Response
{ "safe": true, "flagged_patterns": [], "score": 0.02 }
```

### POST `/detect/fake-node`
```json
// Request
{ "address": "db1x...", "staked": 0.0, "is_validator": true, "uptime_percent": 2.0 }

// Response
{ "is_fake": true, "verdict": "FAKE_NODE", "reasons": ["zero_stake","low_uptime"] }
```

### POST `/blacklist`
```json
{ "address": "db1xscammer" }
// Response: { "blacklisted": true, "address": "db1xscammer" }
```

---

## DevilStorage API (Port 8548)

### GET `/stats`
```json
{
  "node": "db1xstorage001",
  "total_files": 42,
  "capacity_bytes": 107374182400,
  "used_bytes": 1024000,
  "usage_percent": 0
}
```

### POST `/store`
```json
// Request
{
  "data_b64": "SGVsbG8gV29ybGQ=",
  "file_name": "hello.txt",
  "owner": "db1xalice...",
  "is_public": true
}
// Response
{ "success": true, "cid": "dvl1abc123...", "size": 11 }
```

### GET `/retrieve/{cid}`
```json
{ "cid": "dvl1abc123...", "data_b64": "SGVsbG8gV29ybGQ=" }
```

---

## Address Format

```
db1x + SHA256(SHA256(pubkey))[:32 hex chars]

Example: db1x3f8a2c1d4e9b7f2a3e8c1d4b9f2a7e8c
```

## Transaction Signature

All transactions signed with **Ed25519**:
```python
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
import json

tx_data = {"from": "db1x...", "to": "db1x...", "amount": 50.0, "gas_fee": 0.01}
message  = json.dumps(tx_data, sort_keys=True).encode()
priv_key = Ed25519PrivateKey.from_private_bytes(bytes.fromhex(private_key_hex))
sig_hex  = priv_key.sign(message).hex()
```

---

<p align="center">
  DevilChain API Reference &nbsp;|&nbsp;
  <a href="https://nexuzy.tech">Nexuzy Lab</a> &nbsp;|&nbsp;
  <a href="https://devilone.in">Devil One</a>
</p>
