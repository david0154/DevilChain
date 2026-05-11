# DevilChain Network — Docker Deployment Guide

> **Developed by [Nexuzy Lab](https://nexuzy.tech) | Powered by [Devil One](https://devilone.in)**

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    DevilChain Docker Stack                  │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │devilchain    │  │devilguard-ai │  │devilstorage  │      │
│  │-node         │  │  :8547       │  │  :8548       │      │
│  │:8545 :8546   │  │  Python AI   │  │  Rust node   │      │
│  │  Rust core   │  └──────────────┘  └──────────────┘      │
│  └──────────────┘                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │devilscan     │  │devilsocial   │  │devilchat     │      │
│  │  :3000       │  │  :3001       │  │  :3002       │      │
│  │  Next.js     │  │  Next.js     │  │  Next.js     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │postgres      │  │redis         │  │mongodb       │      │
│  │  :5432       │  │  :6379       │  │  :27017      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│  ┌──────────────┐  ┌──────────────┐                         │
│  │devilmine     │  │devilbridge   │                         │
│  │  (worker)    │  │  :8549       │                         │
│  └──────────────┘  └──────────────┘                         │
└─────────────────────────────────────────────────────────────┘
```

---

## Commands

```bash
# Full start
bash docker/start.sh all

# Node only
bash docker/start.sh node

# Run tests
bash docker/start.sh test

# Status
bash docker/start.sh status

# Logs
bash docker/start.sh logs

# Stop
bash docker/start.sh stop

# Full clean
bash docker/start.sh clean
```

## Direct Docker Compose

```bash
cd docker

# Start all
docker compose up -d

# Start specific service
docker compose up -d devilchain-node devilguard-ai

# Rebuild
docker compose build --no-cache

# Logs
docker compose logs -f devilchain-node

# Stop
docker compose down

# Stop + remove volumes
docker compose down -v
```

## Production VPS Deployment

```bash
# Ubuntu 22.04
apt update && apt install -y docker.io docker-compose-plugin git
git clone https://github.com/david0154/DevilChain.git
cd DevilChain
bash docker/start.sh all

# Or use genesis script for bare-metal validator
sudo bash scripts/testnet_genesis.sh
```

---

<p align="center">
  DevilChain Docker Guide &nbsp;|&nbsp;
  <a href="https://nexuzy.tech">Nexuzy Lab</a> &nbsp;|&nbsp;
  <a href="https://devilone.in">Devil One</a>
</p>
