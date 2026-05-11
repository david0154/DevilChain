# DevilChain Network — Complete Setup Guide

## Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | 1.77+ | `curl https://sh.rustup.rs -sSf \| sh` |
| Node.js | 20+ | `nvm install 20` |
| Flutter | 3.19+ | [flutter.dev](https://flutter.dev/docs/get-started/install) |
| Docker | 24+ | [docker.com](https://docs.docker.com/get-docker/) |
| Python | 3.11+ | `pyenv install 3.11` |
| Git | Any | `apt install git` |

---

## 1. Clone Repository

```bash
git clone https://github.com/david0154/DevilChain.git
cd DevilChain
```

---

## 2. Blockchain Core (Rust)

```bash
cd core
cargo build --release

# Run node (lite mode)
./target/release/devilchain-node start --mode lite

# Run node (validator mode)
./target/release/devilchain-node start --mode validator

# Generate wallet
./target/release/devilchain-node gen-wallet

# Check status
./target/release/devilchain-node status
```

APIs available after start:
- REST: `http://localhost:8545`
- GraphQL: `http://localhost:8546/graphql`

---

## 3. Smart Contracts (Hardhat)

```bash
cd contracts
npm install

# Compile all contracts
npx hardhat compile

# Run all tests
npx hardhat test

# Deploy to local testnet
npx hardhat run scripts/deploy.js --network localhost

# Deploy to DevilChain testnet
npx hardhat run scripts/deploy.js --network devilchain_testnet
```

---

## 4. DevilX Wallet (Flutter)

```bash
cd wallet
flutter pub get

# Run on Android
flutter run -d android

# Run on iOS
flutter run -d ios

# Build release APK
flutter build apk --release

# Build for Windows
flutter build windows --release

# Build for Linux
flutter build linux --release
```

---

## 5. DevilScan Explorer (Next.js)

```bash
cd explorer
npm install

# Development
npm run dev
# → http://localhost:3000

# Production build
npm run build && npm start

# Environment variables
cp .env.example .env.local
# Set NEXT_PUBLIC_API_URL=http://your-node:8545
```

---

## 6. DevilGuard AI (Python)

```bash
cd ai
pip install -r requirements.txt

# Start AI API service (port 8547)
python api.py

# Start AI node (validator/miner)
python node.py

# Run tests
pytest tests/
```

---

## 7. DevilStorage Node (Rust)

```bash
cd storage
cargo build --release
mkdir -p /data/devil-storage
./target/release/devil-storage-node
# → http://localhost:8548
```

---

## 8. DevilSocial (Next.js)

```bash
cd social
npm install && npm run dev
# → http://localhost:3001
```

## 9. DevilChat (Next.js)

```bash
cd chat
npm install && npm run dev
# → http://localhost:3002
```

---

## 10. Full Stack with Docker Compose

```bash
cd docker

# Start all services
docker-compose up -d

# Check all services
docker-compose ps

# View logs
docker-compose logs -f devilchain-node

# Stop all
docker-compose down
```

Services started:
| Service | URL |
|---|---|
| DevilChain Node | http://localhost:8545 |
| GraphQL API | http://localhost:8546/graphql |
| AI API | http://localhost:8547 |
| Storage Node | http://localhost:8548 |
| DevilScan Explorer | http://localhost:3000 |
| DevilSocial | http://localhost:3001 |
| DevilChat | http://localhost:3002 |
| Redis | localhost:6379 |
| PostgreSQL | localhost:5432 |
| MongoDB | localhost:27017 |

---

## 11. Testnet Genesis (VPS / Server)

```bash
# On Ubuntu 22.04 VPS
sudo bash scripts/testnet_genesis.sh

# Monitor node
journalctl -u devilchain -f

# Test APIs
curl http://localhost:8545/api/status
curl http://localhost:8545/api/block/latest
```

---

## 12. Environment Variables Reference

| Variable | Default | Description |
|---|---|---|
| `RUST_LOG` | `info` | Core node log level |
| `NEXT_PUBLIC_API_URL` | `http://localhost:8545` | Explorer API endpoint |
| `AI_THRESHOLD` | `0.75` | AI risk score cutoff |
| `STORAGE_PATH` | `/data/devil-storage` | Storage node data dir |
| `DB_PATH` | `/data/devilchain` | RocksDB data dir |
| `P2P_PORT` | `30303` | P2P network port |

---

## 13. Node Types Quick Reference

| Type | Min CPU | Min RAM | Min Disk | Min Stake |
|---|---|---|---|---|
| Lite Node | 2 core | 2 GB | 25 GB SSD | None |
| Validator Node | 4 core | 8 GB | 100 GB SSD | 100 DVC |
| AI Node | 4 core | 8 GB | 100 GB SSD | 100 DVC |
| Archive Node | 8 core | 16 GB | 1 TB SSD | None |
| Storage Node | 2 core | 4 GB | 500 GB HDD | None |

---

## 14. Troubleshooting

**Node won't start:**
```bash
journalctl -u devilchain -n 50
cargo build --release  # Re-build
```

**API not responding:**
```bash
curl http://localhost:8545/api/status
ss -tlnp | grep 8545
```

**Explorer blank page:**
```bash
cd explorer && npm run build
echo 'NEXT_PUBLIC_API_URL=http://localhost:8545' > .env.local
npm start
```

**Contract test failures:**
```bash
cd contracts
npx hardhat clean
npx hardhat compile
npx hardhat test --verbose
```
