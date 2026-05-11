#!/usr/bin/env bash
# DevilChain — One-line VPS Deploy Script
# Works on Ubuntu 20.04/22.04/24.04 with as little as 1GB RAM
# Usage: curl -fsSL https://raw.githubusercontent.com/david0154/DevilChain/main/docker/deploy.sh | bash

set -e
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; RESET='\033[0m'; BOLD='\033[1m'

echo -e "${RED}${BOLD}"
echo "  ██████╗ ███████╗██╗   ██╗██╗██╗      ██████╗██╗  ██╗ █████╗ ██╗███╗   ██╗"
echo -e "${RESET}"
echo -e "  ${CYAN}DevilChain Network — Auto Deploy${RESET}"
echo -e "  ${YELLOW}nexuzy.tech${RESET} | ${YELLOW}devilone.in${RESET}\n"

# ── Detect OS ─────────────────────────────────────────────────
if [ -f /etc/os-release ]; then
  . /etc/os-release
  OS=$ID
else
  OS=unknown
fi
echo -e "  OS: $OS"

# ── Install Docker if missing ──────────────────────────────────
if ! command -v docker &>/dev/null; then
  echo -e "  ${YELLOW}Installing Docker...${RESET}"
  if [[ "$OS" == "ubuntu" || "$OS" == "debian" ]]; then
    apt-get update -qq
    apt-get install -y -qq ca-certificates curl gnupg lsb-release
    install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/$OS/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
https://download.docker.com/linux/$OS $(lsb_release -cs) stable" > /etc/apt/sources.list.d/docker.list
    apt-get update -qq
    apt-get install -y -qq docker-ce docker-ce-cli containerd.io docker-compose-plugin
  elif [[ "$OS" == "centos" || "$OS" == "rhel" || "$OS" == "fedora" ]]; then
    yum install -y docker docker-compose-plugin
    systemctl enable --now docker
  else
    curl -fsSL https://get.docker.com | sh
  fi
  echo -e "  ${GREEN}✓ Docker installed${RESET}"
fi

# ── Optimize system for blockchain node ───────────────────────
echo -e "  ${CYAN}Applying system optimizations...${RESET}"
cat >> /etc/sysctl.conf << 'EOF'
# DevilChain optimizations
net.core.somaxconn=1024
net.ipv4.tcp_tw_reuse=1
net.ipv4.ip_local_port_range=1024 65535
vm.swappiness=10
vm.overcommit_memory=1
fs.file-max=65536
EOF
sysctl -p &>/dev/null || true

# ── Setup swap if low RAM ──────────────────────────────────────
RAM_MB=$(free -m | awk '/Mem:/{print $2}')
if [ "$RAM_MB" -lt 2048 ] && ! swapon --show | grep -q swap; then
  echo -e "  ${YELLOW}Low RAM ($RAM_MB MB) — creating 1GB swap...${RESET}"
  fallocate -l 1G /swapfile
  chmod 600 /swapfile
  mkswap /swapfile
  swapon /swapfile
  echo '/swapfile none swap sw 0 0' >> /etc/fstab
  echo -e "  ${GREEN}✓ Swap enabled${RESET}"
fi

# ── Clone repo ────────────────────────────────────────────────
if [ ! -d "/opt/devilchain" ]; then
  echo -e "  ${CYAN}Cloning DevilChain...${RESET}"
  git clone https://github.com/david0154/DevilChain.git /opt/devilchain
else
  echo -e "  ${CYAN}Updating DevilChain...${RESET}"
  cd /opt/devilchain && git pull --rebase
fi
cd /opt/devilchain

# ── Start ─────────────────────────────────────────────────────
echo -e "\n  ${CYAN}Starting DevilChain Network...${RESET}"
bash docker/start.sh all

echo -e "\n${GREEN}${BOLD}═══════════════════════════════════════${RESET}"
echo -e "${GREEN}  ✓ DevilChain deployed successfully!${RESET}"
echo -e "${GREEN}${BOLD}═══════════════════════════════════════${RESET}"
echo -e "  Explorer → http://$(curl -s ifconfig.me 2>/dev/null || echo localhost):3000"
echo -e "  Node API → http://$(curl -s ifconfig.me 2>/dev/null || echo localhost):8545"
echo -e "  To test  → bash docker/start.sh test\n"
