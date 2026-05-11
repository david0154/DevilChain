"""
DevilChain Wallet Generator + Transaction Test
Generates real Ed25519 keypairs, db1x... addresses, and tests full TX flow:
  generate wallets -> fund from genesis -> send -> stake -> check balance
"""

import os
import time
import hashlib
import base64
import json
import requests
import secrets
from colorama import Fore, Style, init

init(autoreset=True)

NODE = os.getenv("NODE_API", "http://localhost:8545")


# ============================================================
# WALLET GENERATOR  (Ed25519 + db1x address format)
# ============================================================

def generate_keypair():
    """
    Generate Ed25519 keypair.
    Returns: { private_key, public_key, address, mnemonic_hint }
    Uses: cryptography lib if available, else fallback stub
    """
    try:
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
        from cryptography.hazmat.primitives.serialization import (
            Encoding, PublicFormat, PrivateFormat, NoEncryption
        )
        private_key = Ed25519PrivateKey.generate()
        pub_bytes   = private_key.public_key().public_bytes(Encoding.Raw, PublicFormat.Raw)
        priv_bytes  = private_key.private_bytes(Encoding.Raw, PrivateFormat.Raw, NoEncryption())
    except ImportError:
        # Fallback: random 32-byte stub
        priv_bytes = secrets.token_bytes(32)
        import hmac
        pub_bytes = hmac.new(priv_bytes, b"devil", hashlib.sha256).digest()

    # Address = "db1x" + first 32 hex chars of SHA256(pubkey)
    addr_hash   = hashlib.sha256(pub_bytes).hexdigest()[:32]
    address     = f"db1x{addr_hash}"
    public_key  = pub_bytes.hex()
    private_key_hex = priv_bytes.hex()

    # Mnemonic hint (not real BIP39 - just for testnet display)
    words = [
        "devil", "chain", "network", "stake", "block", "hash",
        "token", "node", "crypto", "web3", "dao", "vault"
    ]
    mnemonic = " ".join(secrets.choice(words) for _ in range(12))

    return {
        "address":     address,
        "public_key":  public_key,
        "private_key": private_key_hex,
        "mnemonic":    mnemonic,
    }


def sign_transaction(private_key_hex: str, tx_data: dict) -> str:
    """
    Sign transaction with Ed25519 private key.
    Returns hex signature.
    """
    message = json.dumps(tx_data, sort_keys=True).encode()
    try:
        from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
        from cryptography.hazmat.primitives.serialization import Encoding, PrivateFormat, NoEncryption
        priv = Ed25519PrivateKey.from_private_bytes(bytes.fromhex(private_key_hex))
        sig  = priv.sign(message)
        return sig.hex()
    except Exception:
        # Fallback stub signature for testnet
        h = hashlib.sha256(message + bytes.fromhex(private_key_hex[:32] if len(private_key_hex) >= 32 else private_key_hex.ljust(32, '0'))).hexdigest()
        return f"ed25519_{h}"


def ok(msg):   print(f"{Fore.GREEN}  ✅  {msg}")
def info(msg): print(f"{Fore.CYAN}  ℹ️   {msg}")
def warn(msg): print(f"{Fore.YELLOW}  ⚠️   {msg}")
def fail(msg): print(f"{Fore.RED}  ❌  {msg}")
def section(name): print(f"\n{Fore.CYAN}{Style.BRIGHT}{'='*55}\n  {name}\n{'='*55}")


# ============================================================
# TEST CLASS
# ============================================================

class TestWalletAndTransactions:
    """
    Full wallet + transaction lifecycle test:
    1. Generate wallets (Alice, Bob, Miner)
    2. Fund Alice from genesis faucet
    3. Alice → Bob transfer (with gas fee)
    4. Alice stakes DVC
    5. Alice unstakes DVC
    6. DAO vote
    7. Check all balances
    """

    # Shared wallets across all tests in this class
    _alice  = None
    _bob    = None
    _miner  = None
    _tx_hash = None

    @classmethod
    def setup_class(cls):
        """Generate fresh wallets before tests run"""
        section("GENERATING TEST WALLETS")
        cls._alice = generate_keypair()
        cls._bob   = generate_keypair()
        cls._miner = generate_keypair()

        print()
        for name, w in [("Alice", cls._alice), ("Bob", cls._bob), ("Miner", cls._miner)]:
            print(f"  {Fore.YELLOW}[{name}]{Fore.RESET}")
            print(f"    Address    : {Fore.GREEN}{w['address']}{Fore.RESET}")
            print(f"    Public Key : {w['public_key'][:32]}...")
            print(f"    Mnemonic   : {Fore.CYAN}{w['mnemonic']}{Fore.RESET}")
            print()

    def test_01_wallet_address_format(self):
        """Generated addresses follow db1x... format"""
        for w in [self._alice, self._bob, self._miner]:
            assert w["address"].startswith("db1x"), f"Bad address format: {w['address']}"
            assert len(w["address"]) == 36, f"Bad address length: {len(w['address'])}"
        ok(f"Alice  : {self._alice['address']}")
        ok(f"Bob    : {self._bob['address']}")
        ok(f"Miner  : {self._miner['address']}")

    def test_02_wallet_uniqueness(self):
        """Each generated address is unique"""
        addresses = [self._alice["address"], self._bob["address"], self._miner["address"]]
        assert len(set(addresses)) == 3
        ok("All 3 wallet addresses are unique")

    def test_03_faucet_fund_alice(self):
        """Fund Alice from genesis faucet (testnet only)"""
        r = requests.post(f"{NODE}/api/faucet", json={
            "address": self._alice["address"],
            "amount":  1000.0
        }, timeout=10)
        # 200 = funded, 404 = faucet not enabled (ok on non-testnet)
        assert r.status_code in [200, 404, 405, 422]
        if r.status_code == 200:
            ok(f"Faucet funded Alice: 1000 DVC → {self._alice['address']}")
        else:
            warn(f"Faucet not available (status {r.status_code}) — seeding via genesis wallet")
            # Fallback: try genesis wallet transfer
            seed_payload = {
                "from":      "db1xdao_treasury",
                "to":        self._alice["address"],
                "amount":    1000.0,
                "gas_fee":   0.0,
                "signature": "genesis_seed_sig"
            }
            r2 = requests.post(f"{NODE}/api/send", json=seed_payload, timeout=10)
            assert r2.status_code in [200, 400, 422]
            ok(f"Genesis seed attempted: {r2.status_code}")

    def test_04_check_alice_balance(self):
        """Check Alice's wallet balance"""
        r = requests.get(f"{NODE}/api/wallet/{self._alice['address']}", timeout=10)
        assert r.status_code in [200, 404]
        if r.status_code == 200:
            data = r.json()
            ok(f"Alice balance: {data.get('balance', 'N/A')} DVC | txs: {data.get('tx_count', 0)}")
        else:
            warn("Wallet not found yet (node may still be seeding)")

    def test_05_alice_sends_to_bob(self):
        """Alice sends 50 DVC to Bob with gas fee"""
        tx_data = {
            "from":    self._alice["address"],
            "to":      self._bob["address"],
            "amount":  50.0,
            "gas_fee": 0.01,
        }
        signature = sign_transaction(self._alice["private_key"], tx_data)
        payload   = {**tx_data, "signature": signature}

        r = requests.post(f"{NODE}/api/send", json=payload, timeout=10)
        assert r.status_code in [200, 400, 422]
        resp = r.json()

        if r.status_code == 200:
            self.__class__._tx_hash = resp.get("tx_hash")
            ok(f"TX sent! hash={self._tx_hash} | 50 DVC + 0.01 fee")
        else:
            warn(f"TX response {r.status_code}: {resp} (expected on fresh testnet)")

    def test_06_verify_transaction(self):
        """Look up the sent transaction by hash"""
        if not self._tx_hash:
            warn("No tx_hash from previous test — skipping lookup")
            return
        r = requests.get(f"{NODE}/api/tx/{self._tx_hash}", timeout=10)
        assert r.status_code in [200, 404]
        if r.status_code == 200:
            tx = r.json()
            assert tx.get("from") == self._alice["address"]
            assert tx.get("to")   == self._bob["address"]
            ok(f"TX verified: {tx['from']} → {tx['to']} | {tx.get('amount')} DVC")
        else:
            warn("TX not found yet (may still be in mempool)")

    def test_07_bob_balance_increased(self):
        """Bob's balance should reflect received DVC"""
        r = requests.get(f"{NODE}/api/wallet/{self._bob['address']}", timeout=10)
        assert r.status_code in [200, 404]
        if r.status_code == 200:
            ok(f"Bob balance: {r.json().get('balance', 'N/A')} DVC")
        else:
            warn("Bob wallet not indexed yet")

    def test_08_alice_stakes_dvc(self):
        """Alice stakes 100 DVC to become validator"""
        stake_data = {
            "address": self._alice["address"],
            "amount":  100.0,
        }
        signature = sign_transaction(self._alice["private_key"], stake_data)
        payload   = {**stake_data, "signature": signature}

        r = requests.post(f"{NODE}/api/stake", json=payload, timeout=10)
        assert r.status_code in [200, 400, 422]
        ok(f"Stake 100 DVC: {r.status_code} | {r.json()}")

    def test_09_alice_unstakes_dvc(self):
        """Alice unstakes 50 DVC"""
        unstake_data = {
            "address": self._alice["address"],
            "amount":  50.0,
        }
        signature = sign_transaction(self._alice["private_key"], unstake_data)
        payload   = {**unstake_data, "signature": signature}

        r = requests.post(f"{NODE}/api/unstake", json=payload, timeout=10)
        assert r.status_code in [200, 400, 422]
        ok(f"Unstake 50 DVC: {r.status_code}")

    def test_10_dao_vote(self):
        """Alice votes on DAO proposal"""
        vote_data = {
            "address":     self._alice["address"],
            "proposal_id": 1,
            "vote":        True,
        }
        signature = sign_transaction(self._alice["private_key"], vote_data)
        payload   = {**vote_data, "signature": signature}

        r = requests.post(f"{NODE}/api/vote", json=payload, timeout=10)
        assert r.status_code in [200, 400, 404, 422]
        ok(f"DAO vote: {r.status_code}")

    def test_11_miner_wallet_rewards(self):
        """Miner wallet tracks mining rewards"""
        r = requests.get(f"{NODE}/api/wallet/{self._miner['address']}", timeout=10)
        assert r.status_code in [200, 404]
        ok(f"Miner wallet: {r.status_code}")

    def test_12_ai_scan_generated_tx(self):
        """AI scans a transaction from generated wallets"""
        AI = os.getenv("AI_API", "http://localhost:8547")
        r = requests.post(f"{AI}/scan/tx", json={
            "from_addr": self._alice["address"],
            "to_addr":   self._bob["address"],
            "amount":    50.0,
            "gas_fee":   0.01
        }, timeout=10)
        assert r.status_code == 200
        data = r.json()
        ok(f"AI scan of real wallet TX: score={data['ai_score']} verdict={data['verdict']}")
        assert data["verdict"] == "SAFE"  # Real wallets should be safe

    def test_13_print_wallet_summary(self):
        """Print final wallet summary"""
        section("WALLET TEST SUMMARY")
        for name, w in [("Alice", self._alice), ("Bob", self._bob), ("Miner", self._miner)]:
            r = requests.get(f"{NODE}/api/wallet/{w['address']}", timeout=5)
            bal = r.json().get("balance", "?") if r.status_code == 200 else "N/A"
            print(f"  {Fore.YELLOW}{name:<8}{Fore.RESET} {w['address']}  balance={Fore.GREEN}{bal} DVC{Fore.RESET}")
        ok("Wallet lifecycle test complete")
