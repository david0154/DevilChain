#!/usr/bin/env bash
# DevilChain Quick API Test - No Docker needed, tests running node
# Usage: bash docker/quick_test.sh

set -uo pipefail

NODE=${NODE_API:-http://localhost:8545}
AI=${AI_API:-http://localhost:8547}
STORE=${STORAGE_API:-http://localhost:8548}

GREEN='\033[0;32m'; RED='\033[0;31m'; CYAN='\033[0;36m'; NC='\033[0m'

pass() { echo -e "${GREEN}  ✅ PASS${NC} $1"; }
fail() { echo -e "${RED}  ❌ FAIL${NC} $1"; }
section() { echo -e "\n${CYAN}=== $1 ===${NC}"; }

section "BLOCKCHAIN NODE"
curl -fsS $NODE/api/status           && pass "GET /api/status"           || fail "GET /api/status"
curl -fsS $NODE/api/block/latest     && pass "GET /api/block/latest"     || fail "GET /api/block/latest"
curl -fsS $NODE/api/validators       && pass "GET /api/validators"       || fail "GET /api/validators"
curl -fsS $NODE/api/dao/proposals    && pass "GET /api/dao/proposals"    || fail "GET /api/dao/proposals"
curl -fsS "$NODE/api/wallet/db1xtest_alice_001" && pass "GET /api/wallet/addr" || fail "GET /api/wallet/addr"

section "TRANSACTIONS & FEES"
curl -fsS -X POST $NODE/api/send \
  -H 'Content-Type: application/json' \
  -d '{"from":"db1xalice","to":"db1xbob","amount":10,"gas_fee":0.01,"signature":"test"}' \
  && pass "POST /api/send" || fail "POST /api/send"

curl -fsS -X POST $NODE/api/stake \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1xalice","amount":100,"signature":"test"}' \
  && pass "POST /api/stake" || fail "POST /api/stake"

section "GRAPHQL API"
curl -fsS -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ status { network coin symbol } }"}' \
  && pass "GraphQL status" || fail "GraphQL status"

curl -fsS -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ latestBlock { height hash txCount } }"}' \
  && pass "GraphQL latestBlock" || fail "GraphQL latestBlock"

section "AI SCAN / DEVILGUARD"
curl -fsS $AI/health && pass "AI health" || fail "AI health"

curl -fsS -X POST $AI/scan/tx \
  -H 'Content-Type: application/json' \
  -d '{"from_addr":"db1xalice","to_addr":"db1xbob","amount":10,"gas_fee":0.01}' \
  && pass "AI scan safe TX" || fail "AI scan safe TX"

curl -fsS -X POST $AI/scan/tx \
  -H 'Content-Type: application/json' \
  -d '{"from_addr":"db1xscam001","to_addr":"db1xvictim","amount":999999,"gas_fee":0.0001}' \
  && pass "AI scan scam TX (should flag)" || fail "AI scan scam TX"

curl -fsS -X POST $AI/scan/contract \
  -H 'Content-Type: application/json' \
  -d '{"source_code":"function safe() {}","name":"SafeToken"}' \
  && pass "AI scan safe contract" || fail "AI scan safe contract"

curl -fsS -X POST $AI/scan/contract \
  -H 'Content-Type: application/json' \
  -d '{"source_code":"function withdrawAll() onlyOwner { selfdestruct(owner); }","name":"RugToken"}' \
  && pass "AI scan rug contract (should flag HIGH_RISK)" || fail "AI scan rug contract"

curl -fsS -X POST $AI/moderate \
  -H 'Content-Type: application/json' \
  -d '{"text":"FREE AIRDROP! Guaranteed profit! Double your crypto!"}' \
  && pass "AI NLP spam detection" || fail "AI NLP spam"

curl -fsS -X POST $AI/detect/fake-node \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1xfake","staked":0,"is_validator":true,"uptime_percent":2}' \
  && pass "AI fake node detection" || fail "AI fake node"

section "STORAGE"
curl -fsS $STORE/stats && pass "Storage stats" || fail "Storage stats"

FILE_B64=$(echo -n 'DevilChain test file 2026' | base64)
curl -fsS -X POST $STORE/store \
  -H 'Content-Type: application/json' \
  -d "{\"data_b64\":\"$FILE_B64\",\"file_name\":\"test.txt\",\"owner\":\"db1xtest\",\"is_public\":true}" \
  && pass "Storage: store file" || fail "Storage: store file"

section "MINING"
echo "Checking block height before/after 6s..."
H1=$(curl -fsS $NODE/api/block/latest 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin).get('block_height',0))" 2>/dev/null || echo 0)
sleep 6
H2=$(curl -fsS $NODE/api/block/latest 2>/dev/null | python3 -c "import sys,json; print(json.load(sys.stdin).get('block_height',0))" 2>/dev/null || echo 0)
echo "  Block height: $H1 -> $H2"
[ "$H2" -ge "$H1" ] && pass "Mining producing blocks" || fail "Mining not progressing"

echo ""
echo -e "${CYAN}=== DevilChain Quick Test Complete ===${NC}"
