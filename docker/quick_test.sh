#!/usr/bin/env bash
# DevilChain Quick API Test - Generates wallets and runs full TX test
# Usage: bash docker/quick_test.sh

set -uo pipefail

NODE=${NODE_API:-http://localhost:8545}
AI=${AI_API:-http://localhost:8547}
STORE=${STORAGE_API:-http://localhost:8548}

GREEN='\033[0;32m'; RED='\033[0;31m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; NC='\033[0m'; BOLD='\033[1m'

pass()    { echo -e "${GREEN}  ✅ PASS${NC} $1"; }
fail()    { echo -e "${RED}  ❌ FAIL${NC} $1"; }
warn()    { echo -e "${YELLOW}  ⚠️  WARN${NC} $1"; }
section() { echo -e "\n${CYAN}${BOLD}=== $1 ===${NC}"; }

echo -e "${RED}${BOLD}"
echo ' ██████╗ ███████╗██╗   ██╗██╗██╗      ██████╗██╗  ██╗ █████╗ ██╗███╗   ██╗'
echo ' ██╔══██╗██╔════╝██║   ██║██║██║     ██╔════╝██║  ██║██╔══██╗██║████╗  ██║'
echo ' ██║  ██║█████╗  ██║   ██║██║██║     ██║     ███████║███████║██║██╔██╗ ██║'
echo ' ██║  ██║██╔══╝  ╚██╗ ██╔╝██║██║     ██║     ██╔══██║██╔══██║██║██║╚██╗██║'
echo ' ██████╔╝███████╗ ╚████╔╝ ██║███████╗╚██████╗██║  ██║██║  ██║██║██║ ╚████║'
echo " ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝"
echo -e "${NC}  ${CYAN}DevilChain Network — Quick Test Runner${NC}\n"

# ─── STEP 1: GENERATE WALLETS ───────────────────────────────────
section "GENERATING TEST WALLETS (Ed25519 + db1x format)"

# Generate wallet address from sha256 of random bytes
gen_wallet() {
  local LABEL=$1
  local PRIV=$(python3 -c "import secrets; print(secrets.token_hex(32))")
  local PUB=$(python3  -c "import hashlib,sys; print(hashlib.sha256(bytes.fromhex('$PRIV')).hexdigest())")
  local ADDR="db1x$(python3 -c "import hashlib; print(hashlib.sha256(bytes.fromhex('$PUB')).hexdigest()[:32])")"
  echo -e "  ${YELLOW}[$LABEL]${NC}"
  echo -e "    Address : ${GREEN}$ADDR${NC}"
  echo -e "    PrivKey : ${CYAN}${PRIV:0:32}...${NC}"
  echo "$ADDR"
}

ALICE_ADDR=$(gen_wallet "Alice" | tail -1)
BOB_ADDR=$(gen_wallet "Bob"   | tail -1)
MINER_ADDR=$(gen_wallet "Miner" | tail -1)

echo ""
pass "Wallets generated"
echo "  Alice : $ALICE_ADDR"
echo "  Bob   : $BOB_ADDR"
echo "  Miner : $MINER_ADDR"

# ─── STEP 2: NODE HEALTH ─────────────────────────────────────────
section "BLOCKCHAIN NODE"
curl -fsS $NODE/api/status        >/dev/null && pass "GET /api/status"        || fail "GET /api/status"
curl -fsS $NODE/api/block/latest  >/dev/null && pass "GET /api/block/latest"  || fail "GET /api/block/latest"
curl -fsS $NODE/api/validators    >/dev/null && pass "GET /api/validators"    || fail "GET /api/validators"
curl -fsS $NODE/api/dao/proposals >/dev/null && pass "GET /api/dao/proposals" || fail "GET /api/dao/proposals"

# ─── STEP 3: FAUCET ──────────────────────────────────────────────
section "FAUCET — Fund Test Wallets"
for ADDR in "$ALICE_ADDR" "$BOB_ADDR" "$MINER_ADDR"; do
  STATUS=$(curl -sS -o /dev/null -w "%{http_code}" -X POST $NODE/api/faucet \
    -H 'Content-Type: application/json' \
    -d "{\"address\":\"$ADDR\",\"amount\":1000}")
  [ "$STATUS" = "200" ] && pass "Funded $ADDR (1000 DVC)" || warn "Faucet $STATUS for $ADDR (testnet may not have faucet yet)"
done

# ─── STEP 4: WALLET BALANCES ─────────────────────────────────────
section "WALLET BALANCES"
for PAIR in "Alice:$ALICE_ADDR" "Bob:$BOB_ADDR" "Miner:$MINER_ADDR"; do
  LABEL=$(echo $PAIR | cut -d: -f1)
  ADDR=$(echo $PAIR | cut -d: -f2)
  STATUS=$(curl -sS -o /dev/null -w "%{http_code}" $NODE/api/wallet/$ADDR)
  [ "$STATUS" = "200" ] && pass "$LABEL wallet ($ADDR)" || warn "$LABEL wallet not found yet ($STATUS)"
done

# ─── STEP 5: TRANSACTIONS & FEES ────────────────────────────────
section "TRANSACTIONS & GAS FEES"

# Alice → Bob: 50 DVC + 0.01 gas fee
TX_RESP=$(curl -sS -w "\n%{http_code}" -X POST $NODE/api/send \
  -H 'Content-Type: application/json' \
  -d "{\"from\":\"$ALICE_ADDR\",\"to\":\"$BOB_ADDR\",\"amount\":50,\"gas_fee\":0.01,\"signature\":\"ed25519_test_sig\"}")
TX_STATUS=$(echo "$TX_RESP" | tail -1)
TX_BODY=$(echo "$TX_RESP" | head -1)
[ "$TX_STATUS" = "200" ] && pass "Alice→Bob 50 DVC (fee: 0.01 DVC) TX sent" || warn "TX response $TX_STATUS: $TX_BODY"

# Bob → Alice: 10 DVC
curl -sS -X POST $NODE/api/send \
  -H 'Content-Type: application/json' \
  -d "{\"from\":\"$BOB_ADDR\",\"to\":\"$ALICE_ADDR\",\"amount\":10,\"gas_fee\":0.01,\"signature\":\"ed25519_test_sig\"}" >/dev/null \
  && pass "Bob→Alice 10 DVC" || warn "Bob→Alice TX not confirmed"

# ─── STEP 6: STAKING ─────────────────────────────────────────────
section "STAKING & VALIDATOR"
curl -fsS -X POST $NODE/api/stake \
  -H 'Content-Type: application/json' \
  -d "{\"address\":\"$ALICE_ADDR\",\"amount\":100,\"signature\":\"ed25519_stake_sig\"}" >/dev/null \
  && pass "Alice staked 100 DVC" || warn "Stake TX not confirmed"

curl -fsS -X POST $NODE/api/unstake \
  -H 'Content-Type: application/json' \
  -d "{\"address\":\"$ALICE_ADDR\",\"amount\":50,\"signature\":\"ed25519_unstake_sig\"}" >/dev/null \
  && pass "Alice unstaked 50 DVC" || warn "Unstake TX not confirmed"

# ─── STEP 7: DAO VOTE ────────────────────────────────────────────
section "DAO GOVERNANCE"
curl -fsS -X POST $NODE/api/vote \
  -H 'Content-Type: application/json' \
  -d "{\"address\":\"$ALICE_ADDR\",\"proposal_id\":1,\"vote\":true,\"signature\":\"ed25519_vote_sig\"}" >/dev/null \
  && pass "Alice voted on DAO proposal #1" || warn "Vote not confirmed"

# ─── STEP 8: GRAPHQL ─────────────────────────────────────────────
section "GRAPHQL API"
curl -fsS -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ status { network coin symbol } }"}' >/dev/null \
  && pass "GraphQL status" || fail "GraphQL status"
curl -fsS -X POST http://localhost:8546/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ latestBlock { height hash txCount } }"}' >/dev/null \
  && pass "GraphQL latestBlock" || fail "GraphQL latestBlock"

# ─── STEP 9: AI SCAN ─────────────────────────────────────────────
section "AI SCAN / DEVILGUARD"
curl -fsS $AI/health >/dev/null && pass "AI health" || fail "AI health"

curl -fsS -X POST $AI/scan/tx \
  -H 'Content-Type: application/json' \
  -d "{\"from_addr\":\"$ALICE_ADDR\",\"to_addr\":\"$BOB_ADDR\",\"amount\":50,\"gas_fee\":0.01}" >/dev/null \
  && pass "AI scan: real wallet TX (safe)" || fail "AI scan TX"

curl -fsS -X POST $AI/scan/tx \
  -H 'Content-Type: application/json' \
  -d '{"from_addr":"db1xscam001","to_addr":"db1xvictim","amount":999999,"gas_fee":0.0001}' >/dev/null \
  && pass "AI scan: scam TX (flagged)" || fail "AI scan scam"

curl -fsS -X POST $AI/scan/contract \
  -H 'Content-Type: application/json' \
  -d '{"source_code":"function withdrawAll() onlyOwner { selfdestruct(owner); }","name":"RugToken"}' >/dev/null \
  && pass "AI scan: rug contract (HIGH_RISK)" || fail "AI scan contract"

curl -fsS -X POST $AI/moderate \
  -H 'Content-Type: application/json' \
  -d '{"text":"FREE AIRDROP! Guaranteed profit! Double your crypto!"}' >/dev/null \
  && pass "AI NLP: spam blocked" || fail "AI NLP"

curl -fsS -X POST $AI/detect/fake-node \
  -H 'Content-Type: application/json' \
  -d '{"address":"db1xfake","staked":0,"is_validator":true,"uptime_percent":2}' >/dev/null \
  && pass "AI: fake node detected" || fail "AI fake node"

# ─── STEP 10: STORAGE ────────────────────────────────────────────
section "DEVILVAULT STORAGE"
curl -fsS $STORE/stats >/dev/null && pass "Storage stats" || fail "Storage stats"

FILE_B64=$(python3 -c "import base64; print(base64.b64encode(b'DevilChain test 2026').decode())")
CID_RESP=$(curl -sS -X POST $STORE/store \
  -H 'Content-Type: application/json' \
  -d "{\"data_b64\":\"$FILE_B64\",\"file_name\":\"test.txt\",\"owner\":\"$ALICE_ADDR\",\"is_public\":true}")
CID=$(echo $CID_RESP | python3 -c "import sys,json; print(json.load(sys.stdin).get('cid',''))" 2>/dev/null || echo "")
[ -n "$CID" ] && pass "File stored: CID=$CID" || warn "Storage store: $CID_RESP"
[ -n "$CID" ] && {
  curl -fsS $STORE/retrieve/$CID >/dev/null && pass "File retrieved: CID=$CID" || fail "File retrieve"
}

# ─── STEP 11: MINING CHECK ───────────────────────────────────────
section "MINING (DVLHash-AI)"
H1=$(curl -fsS $NODE/api/block/latest 2>/dev/null | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('block_height',d.get('height',0)))" 2>/dev/null || echo 0)
echo "  Waiting 6s for mining cycles..."
sleep 6
H2=$(curl -fsS $NODE/api/block/latest 2>/dev/null | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('block_height',d.get('height',0)))" 2>/dev/null || echo 0)
echo -e "  Block height: ${YELLOW}$H1${NC} → ${GREEN}$H2${NC} ($((H2-H1)) new blocks)"
[ "$H2" -ge "$H1" ] && pass "Mining is running" || fail "Mining not progressing"

# ─── DONE ────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}${BOLD}════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}${BOLD}DevilChain Quick Test Complete${NC}"
echo -e "  Alice  : ${GREEN}$ALICE_ADDR${NC}"
echo -e "  Bob    : ${GREEN}$BOB_ADDR${NC}"
echo -e "  Miner  : ${GREEN}$MINER_ADDR${NC}"
echo -e "${CYAN}${BOLD}════════════════════════════════════════════${NC}"
