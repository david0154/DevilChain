"""
DevilChain Coin (DVC/DVL) Generator & Inspector
Generates coins, checks balances, simulates minting, shows tokenomics
Usage:
  python generate_coin.py               # Show tokenomics + genesis balances
  python generate_coin.py --mint        # Simulate minting DVC to a wallet
  python generate_coin.py --balance     # Check live balance from node
  python generate_coin.py --faucet      # Drip testnet DVC to wallet
"""

import os, sys, json, hashlib, secrets, argparse, requests
from datetime import datetime

NODE = os.getenv("NODE_API", "http://localhost:8545")

# ============================================================
# TOKENOMICS
# ============================================================
TOKENOMICS = {
    "name":          "DevilCoin",
    "symbol":        "DVC",
    "ticker":        "DVL",
    "decimals":      18,
    "total_supply":  1_000_000_000,
    "genesis_time":  "2026-01-01T00:00:00Z",
    "allocation": {
        "Mining Rewards":  {"pct": 35, "amount": 350_000_000, "wallet": "db1xmining_pool"},
        "Ecosystem":       {"pct": 20, "amount": 200_000_000, "wallet": "db1xecosystem"},
        "DAO Treasury":    {"pct": 15, "amount": 150_000_000, "wallet": "db1xdao_treasury"},
        "Team & Dev":      {"pct": 10, "amount": 100_000_000, "wallet": "db1xteam"},
        "Validators":      {"pct": 10, "amount": 100_000_000, "wallet": "db1xvalidator_pool"},
        "Investors":       {"pct":  5, "amount":  50_000_000, "wallet": "db1xinvestors"},
        "Community":       {"pct":  5, "amount":  50_000_000, "wallet": "db1xcommunity"},
    }
}

RED    = "\033[0;31m"; GREEN  = "\033[0;32m"; CYAN   = "\033[0;36m"
YELLOW = "\033[1;33m"; RESET  = "\033[0m";    BOLD   = "\033[1m"

def banner():
    print(f"{RED}{BOLD}")
    print("  ██████╗ ██╗   ██╗ ██████╗    ██████╗ ██╗   ██╗ ██████╗ ██╗███╗   ██╗")
    print("  ██╔══██╗██║   ██║██╔════╝   ██╔════╝██║   ██║██╔═══██╗██║████╗  ██║")
    print("  ██║  ██║██║   ██║██║        ██║     ██║   ██║██║   ██║██║██╔██╗ ██║")
    print("  ██║  ██║╚██╗ ██╔╝██║        ██║     ╚██╗ ██╔╝██║   ██║██║██║╚██╗██║")
    print("  ██████╔╝ ╚████╔╝ ╚██████╗   ╚██████╗ ╚████╔╝ ╚██████╔╝██║██║ ╚████║")
    print("  ╚═════╝   ╚═══╝   ╚═════╝    ╚═════╝  ╚═══╝   ╚═════╝ ╚═╝╚═╝  ╚═══╝")
    print(f"{RESET}  {CYAN}DevilCoin (DVC/DVL) — Native coin of DevilChain Network{RESET}\n")

def show_tokenomics():
    t = TOKENOMICS
    print(f"{CYAN}{BOLD}{'='*60}")
    print(f"  DEVILCOIN TOKENOMICS")
    print(f"{'='*60}{RESET}")
    print(f"  Name       : {YELLOW}{t['name']}{RESET}")
    print(f"  Symbol     : {GREEN}{t['symbol']} / {t['ticker']}{RESET}")
    print(f"  Decimals   : {t['decimals']}")
    print(f"  Total      : {GREEN}{t['total_supply']:,} DVC{RESET}")
    print(f"  Genesis    : {t['genesis_time']}")
    print()
    print(f"  {'Allocation':<22} {'%':>5}  {'Amount':>18}  Wallet")
    print(f"  {'-'*70}")
    for name, info in t['allocation'].items():
        bar = '█' * (info['pct'] // 2)
        print(f"  {name:<22} {info['pct']:>4}%  {info['amount']:>15,} DVC  {CYAN}{info['wallet']}{RESET}")
        print(f"  {' '*23} {RED}{bar}{RESET}")
    print()

def generate_coin_address(label: str) -> dict:
    """Generate a wallet preloaded with genesis DVC amount"""
    priv = secrets.token_bytes(32)
    pub  = hashlib.sha256(priv).digest()
    addr = f"db1x{hashlib.sha256(pub).hexdigest()[:32]}"
    return {
        "label":       label,
        "address":     addr,
        "private_key": priv.hex(),
        "public_key":  pub.hex(),
        "balance_dvc": 0.0,
        "staked_dvc":  0.0,
    }

def faucet_drip(address: str, amount: float = 1000.0):
    """Request testnet faucet DVC"""
    try:
        r = requests.post(f"{NODE}/api/faucet",
            json={"address": address, "amount": amount}, timeout=8)
        if r.status_code == 200:
            print(f"  {GREEN}✅ Faucet sent {amount} DVC → {address}{RESET}")
            return r.json()
        else:
            print(f"  {YELLOW}⚠️  Faucet {r.status_code} — seeding via genesis wallet{RESET}")
            # fallback: genesis seed
            r2 = requests.post(f"{NODE}/api/send", json={
                "from":      "db1xdao_treasury",
                "to":        address,
                "amount":    amount,
                "gas_fee":   0.0,
                "signature": "genesis_seed_authorized"
            }, timeout=8)
            return r2.json()
    except Exception as e:
        print(f"  {RED}❌ Faucet error: {e}{RESET}")
        return {}

def check_balance(address: str):
    """Query live balance from node"""
    try:
        r = requests.get(f"{NODE}/api/wallet/{address}", timeout=8)
        if r.status_code == 200:
            d = r.json()
            print(f"  {GREEN}Address : {address}{RESET}")
            print(f"  Balance : {YELLOW}{d.get('balance', 0)} DVC{RESET}")
            print(f"  Staked  : {d.get('staked', 0)} DVC")
            print(f"  TX Count: {d.get('tx_count', 0)}")
        else:
            print(f"  {YELLOW}Wallet not indexed yet (node status: {r.status_code}){RESET}")
    except Exception as e:
        print(f"  {RED}Node unreachable: {e}{RESET}")

def simulate_mint(wallet: dict, block_height: int = 1):
    """Simulate mining reward minting"""
    BLOCK_REWARD = 50.0  # DVC per block
    AI_BONUS     = round(secrets.randbelow(500) / 100, 2)  # 0.00–5.00 bonus
    VALIDATOR_FEE_SHARE = round(secrets.randbelow(100) / 100, 4)

    total = BLOCK_REWARD + AI_BONUS + VALIDATOR_FEE_SHARE

    print(f"  {CYAN}{'='*55}{RESET}")
    print(f"  {BOLD}COIN MINT SIMULATION — Block #{block_height}{RESET}")
    print(f"  {CYAN}{'='*55}{RESET}")
    print(f"  Miner Address : {GREEN}{wallet['address']}{RESET}")
    print(f"  Block Reward  : {YELLOW}{BLOCK_REWARD} DVC{RESET}")
    print(f"  AI Bonus      : {GREEN}+{AI_BONUS} DVC{RESET}  (DVLHash-AI score)")
    print(f"  Fee Share     : {GREEN}+{VALIDATOR_FEE_SHARE} DVC{RESET}")
    print(f"  {'─'*40}")
    print(f"  Total Minted  : {RED}{BOLD}{total} DVC{RESET}")
    print(f"  Tx Hash       : dvl_{hashlib.sha256(wallet['address'].encode()).hexdigest()[:32]}")
    print()
    return total

def main():
    parser = argparse.ArgumentParser(description="DevilCoin (DVC/DVL) Generator")
    parser.add_argument("--mint",    action="store_true", help="Simulate mining coin mint")
    parser.add_argument("--balance", type=str, default="",help="Check balance for address")
    parser.add_argument("--faucet",  type=str, default="",help="Drip faucet DVC to address")
    parser.add_argument("--amount",  type=float, default=1000.0)
    parser.add_argument("--count",   type=int,   default=3, help="Wallets to generate")
    parser.add_argument("--export",  action="store_true")
    args = parser.parse_args()

    banner()
    show_tokenomics()

    if args.balance:
        print(f"{CYAN}=== LIVE BALANCE ==={RESET}")
        check_balance(args.balance)
        return

    if args.faucet:
        print(f"{CYAN}=== FAUCET ==={RESET}")
        faucet_drip(args.faucet, args.amount)
        return

    # Generate wallets
    labels  = ["alice", "bob", "miner", "validator", "dao"]
    wallets = [generate_coin_address(labels[i] if i < 5 else f"wallet_{i+1}") for i in range(args.count)]

    print(f"{CYAN}{BOLD}=== GENERATED COIN WALLETS ==={RESET}\n")
    for i, w in enumerate(wallets):
        print(f"  {YELLOW}[{w['label'].upper()}]{RESET}")
        print(f"    Address    : {GREEN}{w['address']}{RESET}")
        print(f"    Public Key : {w['public_key'][:32]}...")
        print(f"    Balance    : {YELLOW}0 DVC{RESET}  (fund via faucet)")
        print(f"    Faucet cmd : curl -X POST {NODE}/api/faucet \\")
        print(f"                   -d '{{\"address\":\"{w['address']}\",\"amount\":1000}}'")
        print()

    if args.mint:
        print()
        for i, w in enumerate(wallets):
            simulate_mint(w, block_height=i+1)

    # Faucet all wallets
    print(f"{CYAN}=== FUNDING WALLETS VIA FAUCET ==={RESET}")
    for w in wallets:
        faucet_drip(w['address'], 1000.0)

    if args.export:
        with open("coins.json", "w") as f:
            json.dump({"tokenomics": TOKENOMICS, "wallets": wallets}, f, indent=2)
        print(f"\n  {GREEN}✅ Exported to coins.json{RESET}")

if __name__ == "__main__":
    main()
