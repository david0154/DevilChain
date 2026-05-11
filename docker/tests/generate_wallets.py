"""
DevilChain Wallet Generator - Standalone CLI Tool
Usage:
  python generate_wallets.py              # Generate 3 wallets
  python generate_wallets.py --count 5    # Generate 5 wallets
  python generate_wallets.py --export     # Save to wallets.json
"""

import hashlib
import secrets
import json
import argparse
import sys

try:
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
    from cryptography.hazmat.primitives.serialization import (
        Encoding, PublicFormat, PrivateFormat, NoEncryption
    )
    HAS_CRYPTO = True
except ImportError:
    HAS_CRYPTO = False

MNEMONIC_WORDS = [
    "devil","chain","network","stake","block","hash","token","node",
    "crypto","web3","dao","vault","shard","proof","layer","bridge",
    "wallet","ledger","flame","shadow","forge","sync","byte","pulse"
]

def generate_wallet(label: str = "") -> dict:
    if HAS_CRYPTO:
        priv      = Ed25519PrivateKey.generate()
        pub_bytes = priv.public_key().public_bytes(Encoding.Raw, PublicFormat.Raw)
        priv_hex  = priv.private_bytes(Encoding.Raw, PrivateFormat.Raw, NoEncryption()).hex()
    else:
        priv_bytes = secrets.token_bytes(32)
        import hmac as _hmac
        pub_bytes = _hmac.new(priv_bytes, b"devilchain", hashlib.sha256).digest()
        priv_hex  = priv_bytes.hex()

    addr_hash = hashlib.sha256(pub_bytes).hexdigest()[:32]
    address   = f"db1x{addr_hash}"
    mnemonic  = " ".join(secrets.choice(MNEMONIC_WORDS) for _ in range(12))

    return {
        "label":       label,
        "address":     address,
        "public_key":  pub_bytes.hex(),
        "private_key": priv_hex,
        "mnemonic":    mnemonic,
    }


def print_wallet(w: dict, index: int):
    RED    = "\033[0;31m"
    GREEN  = "\033[0;32m"
    CYAN   = "\033[0;36m"
    YELLOW = "\033[1;33m"
    RESET  = "\033[0m"
    BOLD   = "\033[1m"

    print(f"  ┌─────────────────────────────────────────────────────┐")
    print(f"  │  {BOLD}{RED}DevilChain Wallet #{index}{RESET}  {YELLOW}{w['label']}{RESET}")
    print(f"  ├─────────────────────────────────────────────────────┤")
    print(f"  │  Address    : {GREEN}{w['address']}{RESET}")
    print(f"  │  Public Key : {w['public_key'][:32]}...")
    print(f"  │  Private Key: {CYAN}{w['private_key'][:32]}...{RESET}")
    print(f"  │  Mnemonic   : {YELLOW}{w['mnemonic']}{RESET}")
    print(f"  └─────────────────────────────────────────────────────┘")
    print()


def main():
    parser = argparse.ArgumentParser(description="DevilChain Wallet Generator")
    parser.add_argument("--count",  type=int, default=3,     help="Number of wallets to generate")
    parser.add_argument("--export", action="store_true",     help="Export to wallets.json")
    parser.add_argument("--labels", nargs="+", default=[],  help="Wallet labels e.g. alice bob miner")
    args = parser.parse_args()

    RED   = "\033[0;31m"
    CYAN  = "\033[0;36m"
    RESET = "\033[0m"
    BOLD  = "\033[1m"

    print(f"""
{RED}{BOLD}
  ██████╗ ███████╗██╗   ██╗██╗██╗      ██████╗██╗  ██╗ █████╗ ██╗███╗   ██╗
  ██╔══██╗██╔════╝██║   ██║██║██║     ██╔════╝██║  ██║██╔══██╗██║████╗  ██║
  ██║  ██║█████╗  ██║   ██║██║██║     ██║     ███████║███████║██║██╔██╗ ██║
  ██║  ██║██╔══╝  ╚██╗ ██╔╝██║██║     ██║     ██╔══██║██╔══██║██║██║╚██╗██║
  ██████╔╝███████╗ ╚████╔╝ ██║███████╗╚██████╗██║  ██║██║  ██║██║██║ ╚████║
  ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
{RESET}
  {CYAN}DevilChain Network — Wallet Generator{RESET}
  Ed25519 | db1x Address Format | Testnet
    """)

    labels = args.labels if args.labels else [
        ["alice","bob","miner","validator","dao"][i] if i < 5 else f"wallet_{i+1}"
        for i in range(args.count)
    ]

    wallets = []
    for i in range(args.count):
        label = labels[i] if i < len(labels) else f"wallet_{i+1}"
        w = generate_wallet(label=label)
        wallets.append(w)
        print_wallet(w, i + 1)

    if args.export:
        path = "wallets.json"
        with open(path, "w") as f:
            json.dump(wallets, f, indent=2)
        print(f"  ✅ Exported {len(wallets)} wallets to {path}")
        print(f"  ⚠️  Keep your private keys safe! Never share them.\n")

    print(f"  ─── Faucet these addresses on testnet ───")
    for w in wallets:
        print(f"  curl -X POST http://localhost:8545/api/faucet \\")
        print(f"    -H 'Content-Type: application/json' \\")
        print(f"    -d '{{\"address\":\"{w['address']}\",\"amount\":1000}}'")
        print()


if __name__ == "__main__":
    main()
