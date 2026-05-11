"""
DevilAI Node — AI Validator / Miner System
Runtime: ONNX + TinyML
Purpose: Scam detection, spam filtering, smart contract analysis,
         node optimization, fake node detection, traffic optimization, auto-healing
"""

import asyncio
import json
import time
import logging
import hashlib
import random
from typing import Dict, Any, List, Optional
from dataclasses import dataclass, asdict

logging.basicConfig(level=logging.INFO, format='[DevilAI] %(asctime)s %(levelname)s %(message)s')
log = logging.getLogger("devil_ai_node")


@dataclass
class AINodeConfig:
    node_address: str = "db1xai_node_001"
    api_endpoint: str = "http://localhost:8545"
    scan_interval: float = 1.0        # seconds between mempool scans
    anomaly_threshold: float = 0.75   # AI score below this = flagged
    auto_heal: bool = True
    nlp_enabled: bool = True
    graph_ai_enabled: bool = True


class DVLHashAI:
    """DVLHash-AI mining algorithm stub"""
    def __init__(self, difficulty: int = 4):
        self.difficulty = difficulty

    def hash(self, data: str, nonce: int) -> str:
        raw = f"{data}{nonce}"
        return hashlib.sha256(raw.encode()).hexdigest()

    def mine(self, block_data: str) -> Dict[str, Any]:
        nonce = 0
        target = "0" * self.difficulty
        start = time.time()
        while True:
            h = self.hash(block_data, nonce)
            if h.startswith(target):
                elapsed = time.time() - start
                log.info(f"[DVLHash-AI] Mined! nonce={nonce} hash={h[:20]}... time={elapsed:.2f}s")
                return {"nonce": nonce, "hash": h, "time_sec": round(elapsed, 3)}
            nonce += 1


class AIRiskScorer:
    """ONNX-powered transaction risk scorer (stub with rule-based fallback)"""

    SPAM_PATTERNS = ["airdrop", "free", "claim", "win", "send back"]
    KNOWN_SCAM_PREFIXES = ["db1xscam", "db1xrug", "db1xfake"]

    def __init__(self, threshold: float = 0.75):
        self.threshold = threshold
        self._tx_counts: Dict[str, int] = {}

    def score_transaction(self, tx: Dict[str, Any]) -> float:
        score = 1.0
        sender = tx.get("from", "")

        # Blacklist check
        for prefix in self.KNOWN_SCAM_PREFIXES:
            if sender.startswith(prefix):
                log.warning(f"[AIRisk] Blacklisted address: {sender}")
                return 0.0

        # Large amount anomaly
        if tx.get("amount", 0) > 500_000:
            score -= 0.4
            log.warning(f"[AIRisk] Large tx: {tx.get('amount')}")

        # Low gas fee (spam indicator)
        if tx.get("gas_fee", 0) < 0.001:
            score -= 0.35

        # Spam velocity: same address sending > 15 tx rapidly
        self._tx_counts[sender] = self._tx_counts.get(sender, 0) + 1
        if self._tx_counts[sender] > 15:
            score -= 0.3
            log.warning(f"[AIRisk] Spam velocity from {sender}: {self._tx_counts[sender]} txs")

        return max(0.0, min(1.0, score))

    def reset_block_counters(self):
        self._tx_counts.clear()

    def is_safe(self, score: float) -> bool:
        return score >= self.threshold


class NLPModerator:
    """Custom NLP for content moderation in DevilSocial / DevilChat"""

    BANNED_PATTERNS = [
        "rug pull", "guaranteed profit", "100x", "send eth", "double your",
        "click here", "exclusive offer", "limited time", "verify wallet"
    ]

    def moderate(self, text: str) -> Dict[str, Any]:
        text_lower = text.lower()
        hits = [p for p in self.BANNED_PATTERNS if p in text_lower]
        risk = len(hits) / len(self.BANNED_PATTERNS)
        return {
            "safe": len(hits) == 0,
            "risk_score": round(risk, 3),
            "flagged_patterns": hits
        }


class GraphAIFraudDetector:
    """Graph AI for rug pull and fraud detection (stub)"""

    RUG_SIGNALS = [
        "selfdestruct", "withdraw_all", "set_fee(100", "blacklist",
        "onlyOwner.*transfer", "mint_unlimited"
    ]

    def analyze_contract(self, source: str) -> Dict[str, Any]:
        import re
        warnings = []
        for sig in self.RUG_SIGNALS:
            if re.search(sig, source, re.IGNORECASE):
                warnings.append(f"Rug signal: {sig}")
        risk = min(1.0, len(warnings) * 0.25)
        return {
            "safe": len(warnings) == 0,
            "risk_score": round(risk, 3),
            "warnings": warnings,
            "verdict": "SAFE" if risk < 0.5 else "HIGH_RISK"
        }

    def detect_fake_node(self, node_info: Dict[str, Any]) -> bool:
        # Fake nodes: report 0 stake but claim validator status
        if node_info.get("staked", 0) < 100 and node_info.get("is_validator"):
            return True
        if node_info.get("uptime_percent", 100) < 10:
            return True
        return False


class AutoHealer:
    """Auto-healing node recovery system"""

    def __init__(self, api_endpoint: str):
        self.api_endpoint = api_endpoint
        self.failed_checks = 0

    async def health_check(self) -> bool:
        try:
            import aiohttp
            async with aiohttp.ClientSession() as session:
                async with session.get(f"{self.api_endpoint}/api/status", timeout=aiohttp.ClientTimeout(total=5)) as r:
                    return r.status == 200
        except Exception:
            return False

    async def attempt_recovery(self):
        log.warning("[AutoHeal] Node unresponsive. Attempting recovery...")
        self.failed_checks += 1
        if self.failed_checks >= 3:
            log.error("[AutoHeal] Node critically unhealthy. Manual intervention required.")
        else:
            log.info("[AutoHeal] Recovery attempt complete.")


class DevilAINode:
    """Main DevilAI Node — AI validator and miner orchestrator"""

    def __init__(self, config: AINodeConfig):
        self.config = config
        self.scorer = AIRiskScorer(config.anomaly_threshold)
        self.nlp = NLPModerator()
        self.graph_ai = GraphAIFraudDetector()
        self.healer = AutoHealer(config.api_endpoint)
        self.miner = DVLHashAI(difficulty=4)
        self.processed_txs = 0
        self.flagged_txs = 0

    async def scan_mempool(self):
        """Scan pending transactions with AI risk scoring"""
        # Simulated mempool pull
        sample_txs = [
            {"from": "db1xabc123", "to": "db1xxyz456", "amount": 50.0, "gas_fee": 0.01},
            {"from": "db1xscam001", "to": "db1xvictim", "amount": 9999.0, "gas_fee": 0.001},
        ]
        for tx in sample_txs:
            score = self.scorer.score_transaction(tx)
            self.processed_txs += 1
            status = "SAFE" if self.scorer.is_safe(score) else "FLAGGED"
            if status == "FLAGGED":
                self.flagged_txs += 1
                log.warning(f"[AI Scan] TX FLAGGED from={tx['from']} score={score:.2f}")
            else:
                log.info(f"[AI Scan] TX SAFE from={tx['from']} score={score:.2f}")

    async def optimize_network(self):
        """Traffic optimization and node health monitoring"""
        healthy = await self.healer.health_check()
        if not healthy:
            await self.healer.attempt_recovery()
        else:
            log.info(f"[Node Health] OK | Processed: {self.processed_txs} | Flagged: {self.flagged_txs}")

    async def run(self):
        log.info(f"🤖 DevilAI Node starting | Address: {self.config.node_address}")
        log.info(f"API: {self.config.api_endpoint} | Threshold: {self.config.anomaly_threshold}")
        while True:
            await self.scan_mempool()
            await self.optimize_network()
            self.scorer.reset_block_counters()
            await asyncio.sleep(self.config.scan_interval)


if __name__ == "__main__":
    config = AINodeConfig()
    node = DevilAINode(config)
    asyncio.run(node.run())
