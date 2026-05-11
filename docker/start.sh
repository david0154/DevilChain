#!/usr/bin/env bash
# DevilChain Network - One Command Start
# Usage: bash docker/start.sh [all|node|test|stop|logs|status]
set -euo pipefail

COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_CYAN='\033[0;36m'
COLOR_YELLOW='\033[1;33m'
COLOR_RESET='\033[0m'

COMPOSE="docker compose -f docker/docker-compose.yml"
CMD=${1:-all}

print_banner() {
  echo -e "${COLOR_RED}"
  echo '  ____            _ _  ____ _           _       '
  echo ' |  _ \  _____   _(_) |/ ___| |__   __ _(_)_ __  '
  echo " | | | |/ _ \\ \\ / / | | |   | '_ \\ / _\` | | '_ \\ "
  echo ' | |_| |  __/\ V /| | | |___| | | | (_| | | | | |'
  echo ' |____/ \___| \_/ |_|_|\____|_| |_|\__,_|_|_| |_|'
  echo -e "${COLOR_RESET}"
  echo -e "${COLOR_CYAN}  DevilChain Network - Docker Testing Stack${COLOR_RESET}"
  echo ""
}

wait_healthy() {
  local name=$1
  local url=$2
  echo -n "  Waiting for $name"
  for i in $(seq 1 30); do
    if curl -fsS "$url" >/dev/null 2>&1; then
      echo -e " ${COLOR_GREEN}READY${COLOR_RESET}"
      return 0
    fi
    echo -n "."
    sleep 2
  done
  echo -e " ${COLOR_YELLOW}TIMEOUT${COLOR_RESET}"
}

case "$CMD" in

all)
  print_banner
  echo -e "${COLOR_CYAN}[1/4] Building all Docker images...${COLOR_RESET}"
  $COMPOSE build --parallel

  echo -e "${COLOR_CYAN}[2/4] Starting all services...${COLOR_RESET}"
  $COMPOSE up -d

  echo -e "${COLOR_CYAN}[3/4] Waiting for services to be healthy...${COLOR_RESET}"
  wait_healthy "DevilChain Node" "http://localhost:8545/api/status"
  wait_healthy "DevilGuard AI"   "http://localhost:8547/health"
  wait_healthy "DevilStorage"    "http://localhost:8548/stats"
  wait_healthy "DevilScan"       "http://localhost:3000"

  echo -e "${COLOR_CYAN}[4/4] Services ready!${COLOR_RESET}"
  echo ""
  echo -e "${COLOR_GREEN}  SERVICE MAP:${COLOR_RESET}"
  echo "  ┌──────────────────────────┬─────────────────────────────┐"
  echo "  │ Service                  │ URL                         │"
  echo "  ├──────────────────────────┼─────────────────────────────┤"
  echo "  │ DevilChain Node (REST)   │ http://localhost:8545       │"
  echo "  │ GraphQL API              │ http://localhost:8546/gql   │"
  echo "  │ DevilGuard AI            │ http://localhost:8547       │"
  echo "  │ DevilStorage             │ http://localhost:8548       │"
  echo "  │ DevilBridge              │ http://localhost:8549       │"
  echo "  │ DevilScan Explorer       │ http://localhost:3000       │"
  echo "  │ DevilSocial              │ http://localhost:3001       │"
  echo "  │ DevilChat                │ http://localhost:3002       │"
  echo "  │ PostgreSQL               │ localhost:5432              │"
  echo "  │ Redis                    │ localhost:6379              │"
  echo "  │ MongoDB                  │ localhost:27017             │"
  echo "  └──────────────────────────┴─────────────────────────────┘"
  echo ""
  echo -e "  Run tests : ${COLOR_YELLOW}bash docker/start.sh test${COLOR_RESET}"
  echo -e "  View logs : ${COLOR_YELLOW}bash docker/start.sh logs${COLOR_RESET}"
  echo -e "  Stop all  : ${COLOR_YELLOW}bash docker/start.sh stop${COLOR_RESET}"
  ;;

node)
  print_banner
  echo "Starting blockchain node only..."
  $COMPOSE up -d devilchain-node redis postgres mongodb
  wait_healthy "DevilChain Node" "http://localhost:8545/api/status"
  echo -e "${COLOR_GREEN}Node started!${COLOR_RESET}"
  ;;

test)
  print_banner
  echo -e "${COLOR_CYAN}Running full test suite...${COLOR_RESET}"
  $COMPOSE --profile test run --rm devil-tester
  ;;

stop)
  echo "Stopping all DevilChain services..."
  $COMPOSE down
  echo -e "${COLOR_GREEN}All services stopped.${COLOR_RESET}"
  ;;

status)
  echo -e "${COLOR_CYAN}DevilChain Service Status${COLOR_RESET}"
  $COMPOSE ps
  echo ""
  echo "API Tests:"
  for svc in \
    "Node|http://localhost:8545/api/status" \
    "AI|http://localhost:8547/health" \
    "Storage|http://localhost:8548/stats"; do
    name=$(echo $svc | cut -d'|' -f1)
    url=$(echo $svc | cut -d'|' -f2)
    if curl -fsS "$url" >/dev/null 2>&1; then
      echo -e "  ${COLOR_GREEN}✅ $name${COLOR_RESET}"
    else
      echo -e "  ${COLOR_RED}❌ $name${COLOR_RESET}"
    fi
  done
  ;;

logs)
  $COMPOSE logs -f devilchain-node devilguard-ai
  ;;

clean)
  echo "Removing all containers, volumes, images..."
  $COMPOSE down -v --rmi all
  echo -e "${COLOR_GREEN}Cleaned.${COLOR_RESET}"
  ;;

*)
  echo "Usage: bash docker/start.sh [all|node|test|stop|status|logs|clean]"
  ;;
esac
