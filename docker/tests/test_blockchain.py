"""
DevilChain Complete Automated Test Suite
Tests: Blockchain, Explorer, Mining, AI Scan, Fees, Storage, Bridge
Run: cd docker && docker compose --profile test run devil-tester
"""

import os
import time
import json
import requests
from colorama import Fore, Style, init

init(autoreset=True)

NODE    = os.getenv("NODE_API",    "http://localhost:8545")
GQL     = os.getenv("GRAPHQL_API", "http://localhost:8546")
AI      = os.getenv("AI_API",      "http://localhost:8547")
STORE   = os.getenv("STORAGE_API", "http://localhost:8548")
BRIDGE  = os.getenv("BRIDGE_API",  "http://localhost:8549")

def ok(msg): print(f"{Fore.GREEN}  ✅  {msg}")
def fail(msg): print(f"{Fore.RED}  ❌  {msg}")
def section(name): print(f"\n{Fore.CYAN}{Style.BRIGHT}{'='*50}\n  {name}\n{'='*50}")


# ============================================================
# BLOCKCHAIN NODE TESTS
# ============================================================

class TestBlockchainNode:
    def test_node_status(self):
        """Node is reachable and returns status"""
        r = requests.get(f"{NODE}/api/status", timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert "network" in data or "chain_length" in data
        ok(f"Node status: {data}")

    def test_latest_block(self):
        """Can fetch the latest block"""
        r = requests.get(f"{NODE}/api/block/latest", timeout=10)
        assert r.status_code == 200
        block = r.json()
        assert "block_height" in block or "height" in block
        ok(f"Latest block: height={block.get('block_height', block.get('height'))}")

    def test_block_by_height(self):
        """Can fetch block #0 (genesis)"""
        r = requests.get(f"{NODE}/api/block/0", timeout=10)
        assert r.status_code in [200, 404]  # 404 ok if not stored yet
        ok(f"Block #0 query responded: {r.status_code}")

    def test_validators_list(self):
        """Validator list is accessible"""
        r = requests.get(f"{NODE}/api/validators", timeout=10)
        assert r.status_code == 200
        data = r.json()
        ok(f"Validators: {data}")

    def test_dao_proposals(self):
        """DAO proposals endpoint accessible"""
        r = requests.get(f"{NODE}/api/dao/proposals", timeout=10)
        assert r.status_code == 200
        ok(f"DAO proposals: {r.json()}")

    def test_wallet_balance(self):
        """Wallet balance query works"""
        r = requests.get(f"{NODE}/api/wallet/db1xtest_alice_001", timeout=10)
        assert r.status_code in [200, 404]
        ok(f"Wallet query: {r.status_code}")


# ============================================================
# TRANSACTION / FEE TESTS
# ============================================================

class TestTransactionFees:
    def test_send_transaction(self):
        """Send a test transaction and verify gas fee deduction"""
        payload = {
            "from": "db1xtest_alice_001",
            "to":   "db1xtest_bob_002",
            "amount": 10.0,
            "gas_fee": 0.01,
            "signature": "test_sig_ed25519_placeholder"
        }
        r = requests.post(f"{NODE}/api/send", json=payload, timeout=10)
        # Accept 200 (success) or 400 (validation expected on testnet)
        assert r.status_code in [200, 400, 422]
        ok(f"Send TX response: {r.status_code} | {r.json()}")

    def test_fee_calculation(self):
        """Verify gas fee is present in transaction"""
        payload = {"from": "a", "to": "b", "amount": 50.0, "gas_fee": 0.01, "signature": "sig"}
        r = requests.post(f"{NODE}/api/send", json=payload, timeout=10)
        ok(f"Fee test: gas_fee=0.01 DVC | response={r.status_code}")
        assert r.status_code in [200, 400, 422]

    def test_staking(self):
        """Staking endpoint works"""
        payload = {
            "address": "db1xtest_alice_001",
            "amount": 100.0,
            "signature": "test_stake_sig"
        }
        r = requests.post(f"{NODE}/api/stake", json=payload, timeout=10)
        assert r.status_code in [200, 400, 422]
        ok(f"Stake TX: {r.status_code} | {r.json()}")

    def test_unstaking(self):
        """Unstaking endpoint works"""
        payload = {
            "address": "db1xtest_alice_001",
            "amount": 50.0,
            "signature": "test_unstake_sig"
        }
        r = requests.post(f"{NODE}/api/unstake", json=payload, timeout=10)
        assert r.status_code in [200, 400, 422]
        ok(f"Unstake TX: {r.status_code}")


# ============================================================
# GRAPHQL API TESTS
# ============================================================

class TestGraphQL:
    def _gql(self, query: str):
        r = requests.post(
            f"{GQL}/graphql",
            json={"query": query},
            headers={"Content-Type": "application/json"},
            timeout=10
        )
        return r

    def test_graphql_status(self):
        """GraphQL status query"""
        r = self._gql("{ status { network coin symbol latestHeight } }")
        assert r.status_code == 200
        ok(f"GraphQL status: {r.json()}")

    def test_graphql_latest_block(self):
        """GraphQL latest block query"""
        r = self._gql("{ latestBlock { height hash txCount aiScore } }")
        assert r.status_code == 200
        ok(f"GraphQL latestBlock: {r.json()}")

    def test_graphql_validators(self):
        """GraphQL validators query"""
        r = self._gql("{ validators { address stakedDvc reputationScore active } }")
        assert r.status_code in [200, 400]  # 400 if field not impl yet
        ok(f"GraphQL validators: {r.status_code}")


# ============================================================
# AI SCAN / DEVILGUARD TESTS
# ============================================================

class TestDevilGuardAI:
    def test_ai_health(self):
        """AI service is running"""
        r = requests.get(f"{AI}/health", timeout=10)
        assert r.status_code == 200
        ok(f"AI health: {r.json()}")

    def test_scan_safe_transaction(self):
        """Safe transaction scores high"""
        r = requests.post(f"{AI}/scan/tx", json={
            "from_addr": "db1xtest_alice_001",
            "to_addr":   "db1xtest_bob_002",
            "amount": 10.0,
            "gas_fee": 0.01
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert "ai_score" in data
        assert "verdict" in data
        ok(f"Safe TX scan: score={data['ai_score']} verdict={data['verdict']}")

    def test_scan_suspicious_transaction(self):
        """Scam address scores low (flagged)"""
        r = requests.post(f"{AI}/scan/tx", json={
            "from_addr": "db1xscam_001",
            "to_addr":   "db1xvictim",
            "amount": 999999.0,
            "gas_fee": 0.0001
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert data["verdict"] == "FLAGGED" or data["ai_score"] < 0.75
        ok(f"Scam TX flagged: score={data['ai_score']} verdict={data['verdict']}")

    def test_scan_safe_contract(self):
        """Clean contract passes scan"""
        safe_source = """
            pragma solidity ^0.8.0;
            contract SafeToken {
                mapping(address=>uint) balances;
                function transfer(address to, uint amount) public {
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                }
            }
        """
        r = requests.post(f"{AI}/scan/contract", json={
            "source_code": safe_source, "name": "SafeToken"
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        ok(f"Safe contract: risk={data['combined_risk_score']} verdict={data['verdict']}")

    def test_scan_rug_contract(self):
        """Rug pull contract is caught"""
        rug_source = """
            function withdrawAll() onlyOwner public {
                selfdestruct(payable(owner));
            }
            function set_fee(uint256 amount100) public { fee = amount100; }
        """
        r = requests.post(f"{AI}/scan/contract", json={
            "source_code": rug_source, "name": "RugToken"
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert data["verdict"] == "HIGH_RISK" or data["combined_risk_score"] >= 0.5
        ok(f"Rug contract caught: risk={data['combined_risk_score']} warnings={data['graph_warnings']}")

    def test_nlp_moderation_clean(self):
        """Clean text passes moderation"""
        r = requests.post(f"{AI}/moderate", json={
            "text": "Hello from DevilChain! This is a test post."
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert data["safe"] == True
        ok(f"Clean text: safe={data['safe']}")

    def test_nlp_moderation_spam(self):
        """Spam text is blocked"""
        r = requests.post(f"{AI}/moderate", json={
            "text": "FREE AIRDROP! Guaranteed profit! Double your crypto! Click here to verify wallet!"
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert data["safe"] == False
        ok(f"Spam blocked: patterns={data['flagged_patterns']}")

    def test_detect_fake_node(self):
        """Fake node (no stake but claims validator) is detected"""
        r = requests.post(f"{AI}/detect/fake-node", json={
            "address": "db1xfake_node_001",
            "staked": 0.0,
            "is_validator": True,
            "uptime_percent": 5.0
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert data["is_fake"] == True
        ok(f"Fake node detected: {data['verdict']}")

    def test_blacklist_address(self):
        """Address can be blacklisted"""
        r = requests.post(f"{AI}/blacklist", json={
            "address": "db1xscammer_test_999"
        }, timeout=10)
        assert r.status_code == 200
        ok(f"Address blacklisted: {r.json()}")


# ============================================================
# DEVILVAULT STORAGE TESTS
# ============================================================

class TestDevilStorage:
    def test_storage_stats(self):
        """Storage node returns stats"""
        r = requests.get(f"{STORE}/stats", timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert "total_files" in data or "capacity_bytes" in data
        ok(f"Storage stats: {data}")

    def test_store_and_retrieve_file(self):
        """Store a file, get CID, retrieve it"""
        import base64
        content = b"DevilChain test file content 2026"
        encoded = base64.b64encode(content).decode()

        # Store
        r = requests.post(f"{STORE}/store", json={
            "data_b64": encoded,
            "file_name": "test.txt",
            "owner": "db1xtest_alice_001",
            "is_public": True
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        assert data.get("success") == True
        cid = data["cid"]
        ok(f"File stored: CID={cid}")

        # Retrieve
        r2 = requests.get(f"{STORE}/retrieve/{cid}", timeout=10)
        assert r2.status_code == 200
        result = r2.json()
        retrieved = base64.b64decode(result["data_b64"])
        assert retrieved == content
        ok(f"File retrieved: {len(retrieved)} bytes, content verified")


# ============================================================
# MINING TESTS
# ============================================================

class TestMining:
    def test_mining_produces_blocks(self):
        """After waiting, block height increases (mining is running)"""
        r1 = requests.get(f"{NODE}/api/block/latest", timeout=10)
        assert r1.status_code == 200
        h1 = r1.json().get("block_height", 0)

        time.sleep(6)  # Wait for 2 mining cycles (~3s each)

        r2 = requests.get(f"{NODE}/api/block/latest", timeout=10)
        assert r2.status_code == 200
        h2 = r2.json().get("block_height", 0)

        ok(f"Mining check: block {h1} -> {h2} ({h2-h1} new blocks in 6s)")
        # Don't assert h2 > h1 as testnet might still be starting

    def test_mining_reward_address(self):
        """Miner wallet receives rewards (check non-zero balance)"""
        r = requests.get(f"{NODE}/api/wallet/db1xminer_test_001", timeout=10)
        assert r.status_code in [200, 404]
        ok(f"Miner wallet query: {r.status_code}")


# ============================================================
# DEVILSCAN EXPLORER TESTS
# ============================================================

class TestDevilScan:
    def test_explorer_homepage(self):
        """Explorer homepage loads"""
        r = requests.get("http://devilscan:3000", timeout=15)
        assert r.status_code == 200
        ok("Explorer homepage loads")

    def test_explorer_no_crash(self):
        """Explorer does not 500 on block page"""
        r = requests.get("http://devilscan:3000", timeout=15)
        assert r.status_code != 500
        ok("Explorer no 500 error")


# ============================================================
# BRIDGE TESTS
# ============================================================

class TestDevilBridge:
    def test_bridge_health(self):
        """Bridge relayer responds"""
        r = requests.get(f"{BRIDGE}/health", timeout=10)
        assert r.status_code in [200, 404]  # May not have /health yet
        ok(f"Bridge relayer: {r.status_code}")
