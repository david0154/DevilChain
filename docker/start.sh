#!/usr/bin/env bash
# DevilChain Network - Master Start Script
# Optimized for low-resource VPS (2 CPU / 2GB RAM)
# Developed by Nexuzy Lab | Powered by Devil One
set -e

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; RESET='\033[0m'; BOLD='\033[1m'

CD="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$CD")"
COMPOSE="docker compose -f $CD/docker-compose.yml"

banner() {
  echo -e "${RED}${BOLD}"
  echo "  ██████╗ ███████╗██╗   ██╗██╗██╗      ██████╗██╗  ██╗ █████╗ ██╗███╗   ██╗"
  echo "  ██╔══██╗██╔════╝██║   ██║██║██║     ██╔════╝██║  ██║██╔══██╗██║████╗  ██║"
  echo "  ██║  ██║█████╗  ██║   ██║██║██║     ██║     ███████║███████║██║██╔██╗ ██║"
  echo "  ██║  ██║██╔══╝  ╚██╗ ██╔╝██║██║     ██║     ██╔══██║██╔══██║██║██║╚██╗██║"
  echo "  ██████╔╝███████╗ ╚████╔╝ ██║███████╗╚██████╗██║  ██║██║  ██║██║██║ ╚████║"
  echo "  ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝"
  echo -e "${RESET}  ${CYAN}DevilChain Network — Testnet 2026${RESET}"
  echo -e "  ${YELLOW}Nexuzy Lab${RESET} (nexuzy.tech) | ${YELLOW}Devil One${RESET} (devilone.in)\n"
}

check_deps() {
  for cmd in docker curl; do
    if ! command -v $cmd &>/dev/null; then
      echo -e "${RED}✗ Missing: $cmd${RESET}" && exit 1
    fi
  done
  if ! docker compose version &>/dev/null; then
    echo -e "${RED}✗ Docker Compose v2 required${RESET}" && exit 1
  fi
}

check_resources() {
  echo -e "${CYAN}── System Resources ─────────────────────${RESET}"
  local ram_kb=$(grep MemTotal /proc/meminfo 2>/dev/null | awk '{print $2}' || echo 2097152)
  local ram_mb=$((ram_kb / 1024))
  local cpus=$(nproc 2>/dev/null || echo 2)
  local disk=$(df -BG / 2>/dev/null | awk 'NR==2{print $4}' | tr -d 'G' || echo 10)

  echo -e "  RAM : ${ram_mb}MB  CPUs: ${cpus}  Disk: ${disk}GB free"

  if [ "$ram_mb" -lt 800 ]; then
    echo -e "  ${YELLOW}⚠  Low RAM detected (<800MB). Starting minimal mode (node + AI only).${RESET}"
    export DEVIL_MODE=minimal
  elif [ "$ram_mb" -lt 1500 ]; then
    echo -e "  ${YELLOW}⚠  Limited RAM (<1.5GB). Skipping miner. Use 'mining' profile manually.${RESET}"
    export DEVIL_MODE=lite
  else
    echo -e "  ${GREEN}✓  Resources OK${RESET}"
    export DEVIL_MODE=full
  fi
  echo
}

wait_healthy() {
  local name=$1 url=$2 max=${3:-60} i=0
  echo -ne "  Waiting for ${CYAN}$name${RESET} "
  until curl -sf "$url" &>/dev/null; do
    sleep 2; i=$((i+2))
    echo -ne "."
    [ $i -ge $max ] && { echo -e " ${RED}TIMEOUT${RESET}"; return 1; }
  done
  echo -e " ${GREEN}✓${RESET}"
}

print_urls() {
  echo -e "\n${GREEN}${BOLD}═══════════════════════════════════════════${RESET}"
  echo -e "${GREEN}  DevilChain Network — All Services Ready${RESET}"
  echo -e "${GREEN}${BOLD}═══════════════════════════════════════════${RESET}"
  echo -e "  ${CYAN}Node REST API ${RESET}   → http://localhost:8545"
  echo -e "  ${CYAN}GraphQL       ${RESET}   → http://localhost:8546/graphql"
  echo -e "  ${CYAN}DevilGuard AI ${RESET}   → http://localhost:8547"
  echo -e "  ${CYAN}DevilStorage  ${RESET}   → http://localhost:8548"
  echo -e "  ${CYAN}DevilBridge   ${RESET}   → http://localhost:8549"
  echo -e "  ${CYAN}DevilScan     ${RESET}   → http://localhost:3000"
  echo -e "  ${CYAN}DevilSocial   ${RESET}   → http://localhost:3001"
  echo -e "  ${CYAN}DevilChat     ${RESET}   → http://localhost:3002"
  echo -e "${GREEN}${BOLD}═══════════════════════════════════════════${RESET}\n"
}

cmd_start() {
  banner; check_deps; check_resources
  echo -e "${CYAN}── Building & Starting Services ─────────${RESET}"

  if [ "$DEVIL_MODE" = "minimal" ]; then
    $COMPOSE up -d --build devilchain-node devilguard-ai redis
  elif [ "$DEVIL_MODE" = "lite" ]; then
    $COMPOSE up -d --build devilchain-node devilguard-ai devilstorage devilbridge devilscan redis
  else
    $COMPOSE up -d --build
  fi

  wait_healthy "node"    "http://localhost:8545/api/status"
  wait_healthy "AI"      "http://localhost:8547/health"      40
  wait_healthy "storage" "http://localhost:8548/stats"       30
  wait_healthy "bridge"  "http://localhost:8549/health"      30
  wait_healthy "explorer" "http://localhost:3000"            60

  print_urls
}

cmd_node() {
  banner; check_deps
  echo -e "${CYAN}Starting node + AI only (minimal)...${RESET}"
  $COMPOSE up -d --build devilchain-node devilguard-ai redis
  wait_healthy "node" "http://localhost:8545/api/status"
  echo -e "  ${GREEN}✓ Node ready at http://localhost:8545${RESET}\n"
}

cmd_test() {
  banner; check_deps
  echo -e "${CYAN}── Running DevilChain Test Suite ────────${RESET}\n"
  # Ensure node is up
  if ! curl -sf http://localhost:8545/api/status &>/dev/null; then
    echo -e "  ${YELLOW}Node not running — starting node first...${RESET}"
    cmd_node
  fi
  # Run pytest in container
  docker run --rm --network host \
    -e NODE_API=http://localhost:8545 \
    -e AI_API=http://localhost:8547 \
    -e STORAGE_API=http://localhost:8548 \
    -v "$CD/tests:/tests" \
    python:3.11-slim bash -c "
      pip install -q requests pytest colorama cryptography &&
      pytest /tests/ -v --tb=short --color=yes -x
    "
}

cmd_status() {
  echo -e "${CYAN}── Container Status ─────────────────────${RESET}"
  $COMPOSE ps
  echo
  echo -e "${CYAN}── Resource Usage ───────────────────────${RESET}"
  docker stats --no-stream --format \
    "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}" \
    2>/dev/null | head -20
  echo
  echo -e "${CYAN}── Health Checks ────────────────────────${RESET}"
  for ep in \
    "Node:http://localhost:8545/api/status" \
    "AI:http://localhost:8547/health" \
    "Storage:http://localhost:8548/stats" \
    "Bridge:http://localhost:8549/health" \
    "Explorer:http://localhost:3000"; do
    name=${ep%%:*}; url=${ep#*:}
    if curl -sf "$url" &>/dev/null; then
      echo -e "  ${GREEN}✓ $name${RESET}"
    else
      echo -e "  ${RED}✗ $name (down)${RESET}"
    fi
  done
}

cmd_logs() {
  $COMPOSE logs -f --tail=50 devilchain-node devilguard-ai
}

cmd_stop()  { $COMPOSE stop; echo -e "${GREEN}All services stopped.${RESET}"; }
cmd_clean() {
  $COMPOSE down -v --remove-orphans
  docker system prune -f
  echo -e "${GREEN}All containers + volumes removed.${RESET}"
}

case "${1:-all}" in
  all|start) cmd_start  ;;
  node)      cmd_node   ;;
  test)      cmd_test   ;;
  status)    cmd_status ;;
  logs)      cmd_logs   ;;
  stop)      cmd_stop   ;;
  clean)     cmd_clean  ;;
  *) echo -e "Usage: $0 {all|node|test|status|logs|stop|clean}" ;;
esac
