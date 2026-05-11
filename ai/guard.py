"""
DevilGuard AI - Security Engine
Detects: rug pulls, scam tokens, bots, spam, fake wallets, anomalies
"""

import re
from typing import Dict, Any

# Suspicious Solidity patterns for rug pull detection
RUG_PATTERNS = [
    r"selfdestruct",
    r"withdraw_all",
    r"onlyOwner.*transfer.*balanceOf",
    r"setMaxWallet.*0",
    r"blacklist",
]

SPAM_THRESHOLD = 10  # Same address sending > 10 tx in 1 block


class DevilGuardAI:
    def __init__(self):
        self.blacklist: set = set()
        self.tx_count: Dict[str, int] = {}

    def analyze_contract(self, source_code: str) -> Dict[str, Any]:
        """Scan contract source for rug pull indicators"""
        warnings = []
        for pattern in RUG_PATTERNS:
            if re.search(pattern, source_code, re.IGNORECASE):
                warnings.append(f"Suspicious pattern: {pattern}")
        return {
            "safe": len(warnings) == 0,
            "warnings": warnings,
            "risk_score": min(1.0, len(warnings) * 0.3)
        }

    def analyze_transaction(self, tx: Dict[str, Any]) -> Dict[str, Any]:
        """Detect anomalous transactions"""
        warnings = []
        sender = tx.get("from", "")

        if sender in self.blacklist:
            warnings.append("Sender is blacklisted")

        if tx.get("amount", 0) > 1_000_000:
            warnings.append("Unusually large transaction")

        # Bot/spam detection
        self.tx_count[sender] = self.tx_count.get(sender, 0) + 1
        if self.tx_count[sender] > SPAM_THRESHOLD:
            warnings.append(f"Spam detected: {self.tx_count[sender]} tx from same address")

        return {
            "safe": len(warnings) == 0,
            "warnings": warnings,
            "risk_score": min(1.0, len(warnings) * 0.25)
        }

    def blacklist_address(self, address: str):
        self.blacklist.add(address)

    def reset_block_counters(self):
        """Call at each new block"""
        self.tx_count.clear()


if __name__ == "__main__":
    guard = DevilGuardAI()

    # Test contract
    result = guard.analyze_contract("function withdrawAll() onlyOwner { selfdestruct(owner); }")
    print("Contract scan:", result)

    # Test transaction
    tx = {"from": "db1xabc", "to": "db1xxyz", "amount": 50, "gas_fee": 0.01}
    result = guard.analyze_transaction(tx)
    print("TX scan:", result)
