# DevilChain Network — Deployment Guide

> **Developed by [Nexuzy Lab](https://nexuzy.tech) | Powered by [Devil One](https://devilone.in)**

---

## Minimum Requirements

| Tier | CPU | RAM | Disk | Services |
|---|---|---|---|---|
| **Minimal** | 1 Core | 512MB + swap | 5 GB | Node + AI only |
| **Lite** | 1 Core | 1 GB | 10 GB | Node + AI + Storage + Bridge + Explorer |
| **Standard** | 2 Core | 2 GB | 20 GB | All services (no miner) |
| **Full** | 4 Core | 4 GB | 40 GB | All services + miner |

---

## Option 1 — One-Line VPS Deploy (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/david0154/DevilChain/main/docker/deploy.sh | bash
```

This script:
- Installs Docker if missing (Ubuntu/Debian/CentOS/Fedora)
- Creates 1GB swap if RAM < 2GB
- Applies Linux kernel optimizations
- Clones repo + starts all services
- Prints public URLs

---

## Option 2 — Manual Docker

```bash
git clone https://github.com/david0154/DevilChain.git
cd DevilChain
bash docker/start.sh all
```

---

## Option 3 — Node Only (512MB VPS)

```bash
bash docker/start.sh node
# Starts: node + AI + redis only
# Uses ~350MB RAM total
```

---

## Resource Usage Per Service

| Service | CPU Limit | RAM Limit | Image Size |
|---|---|---|---|
| devilchain-node | 0.6 CPU | 400 MB | ~35 MB |
| devilguard-ai | 0.4 CPU | 200 MB | ~90 MB |
| devilstorage | 0.2 CPU | 100 MB | ~60 MB |
| devilbridge | 0.2 CPU | 80 MB | ~60 MB |
| devilscan | 0.3 CPU | 150 MB | ~120 MB |
| devilsocial | 0.2 CPU | 100 MB | ~120 MB |
| devilchat | 0.2 CPU | 100 MB | ~120 MB |
| redis | 0.1 CPU | 32 MB | ~7 MB |
| **TOTAL** | **~2.2 CPU** | **~1.2 GB** | — |

> On a 2GB VPS, all services fit with ~800MB headroom.

---

## Start Commands

```bash
bash docker/start.sh all      # Full stack (auto detects resources)
bash docker/start.sh node     # Node + AI only (minimal)
bash docker/start.sh test     # Run test suite
bash docker/start.sh status   # Show resource usage + health
bash docker/start.sh logs     # Live logs
bash docker/start.sh stop     # Stop all
bash docker/start.sh clean    # Remove everything
```

## Enable Mining (optional)

```bash
# Mining is disabled by default to save resources
# Enable with Docker profile:
docker compose -f docker/docker-compose.yml --profile mining up -d
```

---

## Auto-Scaling: Resource Detection

`start.sh` auto-detects RAM and adjusts:

| RAM | Mode | Services Started |
|---|---|---|
| < 800 MB | Minimal | node + AI + redis |
| 800MB – 1.5GB | Lite | + storage + bridge + explorer |
| > 1.5 GB | Full | All 8 services |

---

## Production VPS Recommended

```
Provider: Hetzner CX21 / DigitalOcean Basic / Vultr Regular
OS:       Ubuntu 22.04 LTS
CPU:      2 vCPU
RAM:      4 GB
Disk:     40 GB SSD
Cost:     ~$6–12/month
```

---

<p align="center">
  DevilChain Deploy Guide &nbsp;|&nbsp;
  <a href="https://nexuzy.tech">Nexuzy Lab</a> &nbsp;|&nbsp;
  <a href="https://devilone.in">Devil One</a>
</p>
