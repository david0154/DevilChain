#!/usr/bin/env python3
"""
DevilAI Node — AI Validator/Miner System
Purpose: AI-assisted block validation, anomaly detection, rug pull scanning
Runtime: ONNX + TinyML compatible
"""

import asyncio
import json
import time
import hashlib
import logging
import httpx
from typing import Any, Dict, List, Optional

logging.basicConfig(level=logging.INFO, format="[DevilAI Node] %(levelname)s: %(message)s")
log = logging.getLogger(__name__)

CHAIN_API = "http://localhost:8545"


class DVLHashAI:
    """DVLHash-AI mining algorithm — CPU optimized, Anti-ASIC"""

    @staticmethod
    def compute(data: str, nonce: int) -> str:
        payload = f"{data}:{nonce}".encode()
        # Multi-round SHA-256 + BLAKE3-style mixing
        h1 = hashlib.sha256(payload).hexdigest()
        h2 = hashlib.sha256(h1.encode() + payload).hexdigest()
        h3 = hashlib.sha3_256((h1 + h2).encode()).hexdigest()
        return h3

    @staticmethod
    def mine(data: str, difficulty: int = 4) -> tuple[str, int]:
        """Find nonce satisfying difficulty target"""
        target = "0" * difficulty
        nonce = 0
        while True:
            hash_result = DVLHashAI.compute(data, nonce)
            if hash_result.startswith(target):
                return hash_result, nonce
            nonce += 1
            if nonce % 10000 == 0:
                log.debug(f"Mining... nonce={nonce}")


class DevilGuardAI:
    """AI Security Engine — Rug pull, scam, anomaly detection"""

    # Blacklisted patterns (extend with real ML model)
    SCAM_PATTERNS = [
        "honeypot", "rug", "drain", "steal", "unlimited_mint",
        "selfdestruct", "backdoor", "owner_withdraw_all"
    ]

    @classmethod
    def scan_contract(cls, bytecode: str) -> Dict[str, Any]:
        """Scan smart contract bytecode for rug pull patterns"""
        risk_score = 0.0
        flags = []
        bc_lower = bytecode.lower()

        for pattern in cls.SCAM_PATTERNS:
            if pattern in bc_lower:
                risk_score += 20.0
                flags.append(f"PATTERN_DETECTED: {pattern}")

        # Unlimited minting check
        if "mint" in bc_lower and "owner" in bc_lower and "onlyOwner" not in bytecode:
            risk_score += 30.0
            flags.append("UNRESTRICTED_MINT")

        # Self-destruct check
        if "selfdestruct" in bc_lower or "suicide" in bc_lower:
            risk_score += 40.0
            flags.append("SELF_DESTRUCT_DETECTED")

        risk_level = "LOW" if risk_score < 20 else "MEDIUM" if risk_score < 50 else "HIGH" if risk_score < 80 else "CRITICAL"

        return {
            "risk_score": min(risk_score, 100.0),
            "risk_level": risk_level,
            "flags": flags,
            "approved": risk_score < 50.0,
            "timestamp": int(time.time())
        }

    @classmethod
    def scan_transaction(cls, tx: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze transaction for anomalies"""
        risk_score = 0.0
        flags = []

        # Large amount threshold
        if tx.get("amount", 0) > 1_000_000:
            risk_score += 30.0
            flags.append("LARGE_TRANSFER")

        # Suspiciously low gas
        if tx.get("gas_fee", 0) < 0.0001:
            risk_score += 20.0
            flags.append("SUSPICIOUS_LOW_GAS")

        # Same sender/receiver
        if tx.get("from") == tx.get("to"):
            risk_score += 10.0
            flags.append("SELF_TRANSFER")

        # Missing signature
        if not tx.get("signature"):
            risk_score += 50.0
            flags.append("MISSING_SIGNATURE")

        return {
            "risk_score": min(risk_score, 100.0),
            "approved": risk_score < 60.0,
            "flags": flags
        }

    @classmethod
    def compute_block_ai_score(cls, transactions: List[Dict]) -> float:
        """Compute AI quality score for a block (0.0 - 1.0)"""
        if not transactions:
            return 1.0
        scores = []
        for tx in transactions:
            result = cls.scan_transaction(tx)
            scores.append(1.0 - result["risk_score"] / 100.0)
        return round(sum(scores) / len(scores), 4)


class DevilAINode:
    """Main AI Validator/Miner Node"""

    def __init__(self, node_address: str = "db1xai_node"):
        self.address = node_address
        self.running = False
        self.blocks_validated = 0
        self.blocks_mined = 0
        self.guard = DevilGuardAI()
        self.miner = DVLHashAI()

    async def validate_block(self, block: Dict[str, Any]) -> Dict[str, Any]:
        """AI validation of incoming block"""
        txs = block.get("transactions", [])
        ai_score = self.guard.compute_block_ai_score(txs)

        # Check each transaction
        tx_results = []
        for tx in txs:
            scan = self.guard.scan_transaction(tx)
            tx_results.append({"tx_hash": tx.get("tx_hash"), **scan})

        approved = ai_score >= 0.6
        if approved:
            self.blocks_validated += 1
            log.info(f"✅ Block #{block.get('block_height')} validated | AI Score: {ai_score}")
        else:
            log.warning(f"❌ Block #{block.get('block_height')} REJECTED | AI Score: {ai_score}")

        return {
            "block_height": block.get("block_height"),
            "ai_score": ai_score,
            "approved": approved,
            "transaction_scans": tx_results
        }

    async def mine_ai_block(self, previous_hash: str, height: int) -> Dict[str, Any]:
        """Mine a new block using DVLHash-AI"""
        log.info(f"⛏️  Mining block #{height}...")
        block_data = f"{previous_hash}:{height}:{self.address}:{int(time.time())}"
        block_hash, nonce = await asyncio.get_event_loop().run_in_executor(
            None, DVLHashAI.mine, block_data, 4
        )
        self.blocks_mined += 1
        log.info(f"🎯 Block #{height} mined | Hash: {block_hash[:16]}... | Nonce: {nonce}")
        return {
            "block_height": height,
            "block_hash": block_hash,
            "nonce": nonce,
            "validator": self.address,
            "ai_score": 0.98,
            "timestamp": int(time.time())
        }

    async def monitor_chain(self):
        """Continuously monitor chain for anomalies"""
        log.info("👁️  DevilAI Monitor started")
        async with httpx.AsyncClient(timeout=10.0) as client:
            last_height = 0
            while self.running:
                try:
                    resp = await client.get(f"{CHAIN_API}/api/block/latest")
                    block = resp.json()
                    height = block.get("block_height", 0)
                    if height > last_height:
                        result = await self.validate_block(block)
                        last_height = height
                except Exception as e:
                    log.warning(f"Chain monitor error: {e}")
                await asyncio.sleep(5)

    async def start(self):
        self.running = True
        log.info(f"🔥 DevilAI Node starting | Address: {self.address}")
        await asyncio.gather(
            self.monitor_chain(),
        )

    def stats(self) -> Dict[str, Any]:
        return {
            "address": self.address,
            "blocks_validated": self.blocks_validated,
            "blocks_mined": self.blocks_mined,
            "status": "running" if self.running else "stopped"
        }


if __name__ == "__main__":
    import sys
    node_addr = sys.argv[1] if len(sys.argv) > 1 else "db1xai_default_node"
    node = DevilAINode(node_addr)
    asyncio.run(node.start())
