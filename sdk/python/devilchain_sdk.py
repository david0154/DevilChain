"""
DevilChain Python SDK
Connect to DevilChain Network from Python / AI backends
"""

import requests
from typing import Optional, Dict, Any

class DevilChainSDK:
    def __init__(self, api_base: str = "http://localhost:8545"):
        self.api_base = api_base.rstrip("/")

    def _get(self, path: str) -> Dict[str, Any]:
        r = requests.get(f"{self.api_base}{path}")
        r.raise_for_status()
        return r.json()

    def _post(self, path: str, data: dict) -> Dict[str, Any]:
        r = requests.post(f"{self.api_base}{path}", json=data)
        r.raise_for_status()
        return r.json()

    def get_status(self) -> Dict[str, Any]:
        """Get DevilChain network status"""
        return self._get("/api/status")

    def get_latest_block(self) -> Dict[str, Any]:
        """Get the latest block"""
        return self._get("/api/block/latest")

    def get_block(self, height: int) -> Dict[str, Any]:
        """Get block by height"""
        return self._get(f"/api/block/{height}")

    def get_transaction(self, tx_hash: str) -> Dict[str, Any]:
        """Get transaction by hash"""
        return self._get(f"/api/tx/{tx_hash}")

    def get_wallet(self, address: str) -> Dict[str, Any]:
        """Get wallet info"""
        return self._get(f"/api/wallet/{address}")

    def send_transaction(self, from_addr: str, to_addr: str,
                         amount: float, gas_fee: float,
                         signature: str) -> Dict[str, Any]:
        """Send a DVC transaction"""
        return self._post("/api/send", {
            "from": from_addr, "to": to_addr,
            "amount": amount, "gas_fee": gas_fee,
            "signature": signature
        })

    def stake(self, address: str, amount: float, signature: str) -> Dict[str, Any]:
        """Stake DVC for validator"""
        return self._post("/api/stake", {
            "address": address, "amount": amount, "signature": signature
        })

    def get_validators(self) -> Dict[str, Any]:
        """List active validators"""
        return self._get("/api/validators")

    def get_dao_proposals(self) -> Dict[str, Any]:
        """Get DAO governance proposals"""
        return self._get("/api/dao/proposals")


if __name__ == "__main__":
    sdk = DevilChainSDK()
    print(sdk.get_status())
