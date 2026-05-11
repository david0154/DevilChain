"""
DevilAI FastAPI Service
Endpoints for AI risk scoring, contract analysis, NLP moderation
Port: 8547
"""

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import Dict, Any, Optional, List
from guard import DevilGuardAI
from node import AIRiskScorer, NLPModerator, GraphAIFraudDetector
import uvicorn
import logging

logging.basicConfig(level=logging.INFO)
log = logging.getLogger("devil_ai_api")

app = FastAPI(
    title="DevilGuard AI API",
    description="AI Security Engine for DevilChain Network",
    version="1.0.0"
)

# Singletons
guard = DevilGuardAI()
scorer = AIRiskScorer()
nlp = NLPModerator()
graph_ai = GraphAIFraudDetector()


class TransactionIn(BaseModel):
    tx_hash: Optional[str] = None
    from_addr: str
    to_addr: str
    amount: float
    gas_fee: float
    timestamp: Optional[int] = None


class ContractIn(BaseModel):
    source_code: str
    name: Optional[str] = "Unknown"


class TextIn(BaseModel):
    text: str
    context: Optional[str] = "general"


class AddressIn(BaseModel):
    address: str


class NodeInfoIn(BaseModel):
    address: str
    staked: float
    is_validator: bool
    uptime_percent: float


@app.get("/")
def root():
    return {
        "service": "DevilGuard AI",
        "network": "DevilChain",
        "version": "1.0.0",
        "endpoints": ["/scan/tx", "/scan/contract", "/scan/address", "/moderate", "/detect/fake-node"]
    }


@app.get("/health")
def health():
    return {"status": "ok", "service": "DevilGuard AI"}


@app.post("/scan/tx")
def scan_transaction(tx: TransactionIn):
    """AI risk score a transaction"""
    tx_dict = {
        "from": tx.from_addr,
        "to": tx.to_addr,
        "amount": tx.amount,
        "gas_fee": tx.gas_fee
    }
    score = scorer.score_transaction(tx_dict)
    guard_result = guard.analyze_transaction(tx_dict)
    return {
        "ai_score": round(score, 4),
        "safe": scorer.is_safe(score),
        "guard_warnings": guard_result.get("warnings", []),
        "verdict": "SAFE" if scorer.is_safe(score) else "FLAGGED"
    }


@app.post("/scan/contract")
def scan_contract(contract: ContractIn):
    """Analyze smart contract for rug pulls, scams"""
    guard_result = guard.analyze_contract(contract.source_code)
    graph_result = graph_ai.analyze_contract(contract.source_code)
    combined_risk = (guard_result["risk_score"] + graph_result["risk_score"]) / 2
    return {
        "contract_name": contract.name,
        "safe": combined_risk < 0.5,
        "combined_risk_score": round(combined_risk, 4),
        "guard_warnings": guard_result["warnings"],
        "graph_warnings": graph_result["warnings"],
        "verdict": "SAFE" if combined_risk < 0.5 else "HIGH_RISK"
    }


@app.post("/scan/address")
def scan_address(body: AddressIn):
    """Check if address is flagged as fake/spam"""
    is_spam = guard.detect_spam_address(body.address)
    blacklisted = body.address in guard.blacklist
    return {
        "address": body.address,
        "is_spam": is_spam,
        "blacklisted": blacklisted,
        "safe": not is_spam and not blacklisted
    }


@app.post("/moderate")
def moderate_content(body: TextIn):
    """NLP content moderation for DevilSocial/DevilChat"""
    result = nlp.moderate(body.text)
    return {
        "text_preview": body.text[:80],
        "safe": result["safe"],
        "risk_score": result["risk_score"],
        "flagged_patterns": result["flagged_patterns"],
        "action": "ALLOW" if result["safe"] else "BLOCK"
    }


@app.post("/detect/fake-node")
def detect_fake_node(node: NodeInfoIn):
    """Detect fake/malicious validator nodes"""
    node_dict = {
        "address": node.address,
        "staked": node.staked,
        "is_validator": node.is_validator,
        "uptime_percent": node.uptime_percent
    }
    is_fake = graph_ai.detect_fake_node(node_dict)
    return {
        "address": node.address,
        "is_fake": is_fake,
        "verdict": "FAKE_NODE" if is_fake else "LEGITIMATE",
        "recommendation": "Remove from validator set" if is_fake else "OK"
    }


@app.post("/blacklist")
def blacklist_address(body: AddressIn):
    """Add address to AI blacklist"""
    guard.blacklist_address(body.address)
    return {"address": body.address, "action": "BLACKLISTED"}


if __name__ == "__main__":
    uvicorn.run("api:app", host="0.0.0.0", port=8547, reload=False)
