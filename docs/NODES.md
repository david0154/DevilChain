# DevilChain Node Setup Guide

## Node Types

| Node | Purpose |
|---|---|
| Lite Node | Small VPS, lightweight participation |
| Validator Node | Staking-based block validation |
| AI Node | AI moderation and security |
| Archive Node | Full blockchain history |
| Storage Node | Decentralized file storage |

---

## Lite Node Requirements

| Resource | Minimum |
|---|---|
| CPU | 2 Cores |
| RAM | 2 GB |
| Storage | 25 GB SSD |
| OS | Ubuntu 22.04 |
| Network | Stable broadband |

## Validator Node Requirements

| Resource | Minimum |
|---|---|
| CPU | 4 Cores |
| RAM | 8 GB |
| Storage | 100 GB SSD |
| OS | Ubuntu 22.04 |
| Stake | Minimum DVC |

---

## Running a Lite Node (Quick Start)

```bash
# 1. Install dependencies
apt update && apt install -y curl git build-essential

# 2. Download DevilChain node binary
curl -L https://github.com/david0154/DevilChain/releases/latest/download/devilchain-node -o devilchain-node
chmod +x devilchain-node

# 3. Initialize node
./devilchain-node init --type=lite

# 4. Start node
./devilchain-node start
```

## Running a Validator Node

```bash
# Initialize validator
./devilchain-node init --type=validator --stake-address=db1x...

# Register with DAO
./devilchain-node validator register --dao-sig=<your_dao_signature>

# Start validator
./devilchain-node start --mode=validator
```

---

## Mining (DVLHash-AI)

```bash
# Start CPU mining
./devilchain-node mine --threads=4 --wallet=db1x...
```

The `DVLHash-AI` algorithm is:
- CPU optimized
- Anti-ASIC
- Anti-GPU domination
- Dynamically difficulty-adjusted
- AI-assisted optimization
