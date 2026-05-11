#!/usr/bin/env python3
"""
DevilGuard AI FastAPI Service
Port: 8547
Purposes: Contract scanning, TX risk analysis, spam filtering, node health
"""

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Any, Dict, List, Optional
import time
import uvicorn

from node import DevilGuardAI, DVLHashAI

app = FastAPI(
    title="DevilGuard AI API",
    description="DevilChain AI Security Engine — Contract Scanning, TX Analysis, Spam Filtering",
    version="0.1.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

guard = DevilGuardAI()


class ContractScanRequest(BaseModel):
    bytecode: str
    contract_name: Optional[str] = "Unknown"


class TransactionScanRequest(BaseModel):
    tx_hash: str
    from_addr: str
    to_addr: str
    amount: float
    gas_fee: float
    signature: Optional[str] = ""


class BatchTxScanRequest(BaseModel):
    transactions: List[Dict[str, Any]]


class SpamFilterRequest(BaseModel):
    content: str
    sender: str


@app.get("/")
async def root():
    return {
        "service": "DevilGuard AI",
        "version": "0.1.0",
        "network": "DevilChain",
        "endpoints": ["/scan/contract", "/scan/tx", "/scan/batch", "/spam/check", "/health"]
    }


@app.get("/health")
async def health():
    return {"status": "healthy", "timestamp": int(time.time()), "service": "DevilGuard AI"}


@app.post("/scan/contract")
async def scan_contract(req: ContractScanRequest):
    if not req.bytecode.strip():
        raise HTTPException(status_code=400, detail="Bytecode cannot be empty")
    result = guard.scan_contract(req.bytecode)
    result["contract_name"] = req.contract_name
    return result


@app.post("/scan/tx")
async def scan_transaction(req: TransactionScanRequest):
    tx = {
        "tx_hash": req.tx_hash,
        "from": req.from_addr,
        "to": req.to_addr,
        "amount": req.amount,
        "gas_fee": req.gas_fee,
        "signature": req.signature
    }
    result = guard.scan_transaction(tx)
    result["tx_hash"] = req.tx_hash
    return result


@app.post("/scan/batch")
async def scan_batch(req: BatchTxScanRequest):
    results = []
    for tx in req.transactions:
        scan = guard.scan_transaction(tx)
        scan["tx_hash"] = tx.get("tx_hash", "unknown")
        results.append(scan)
    ai_block_score = guard.compute_block_ai_score(req.transactions)
    return {
        "block_ai_score": ai_block_score,
        "total": len(results),
        "approved_count": sum(1 for r in results if r["approved"]),
        "rejected_count": sum(1 for r in results if not r["approved"]),
        "transaction_scans": results
    }


@app.post("/spam/check")
async def spam_check(req: SpamFilterRequest):
    content_lower = req.content.lower()
    spam_keywords = ["free tokens", "100x guaranteed", "send eth get back",
                     "airdrop claim", "private key", "wallet seed", "click here now"]
    spam_score = 0
    flags = []
    for kw in spam_keywords:
        if kw in content_lower:
            spam_score += 25
            flags.append(f"SPAM_KEYWORD: {kw}")
    is_spam = spam_score >= 50
    return {
        "sender": req.sender,
        "is_spam": is_spam,
        "spam_score": min(spam_score, 100),
        "flags": flags
    }


@app.post("/mine/hash")
async def compute_hash(data: Dict[str, Any]):
    block_data = str(data.get("block_data", ""))
    nonce = int(data.get("nonce", 0))
    result = DVLHashAI.compute(block_data, nonce)
    return {"hash": result, "nonce": nonce}


if __name__ == "__main__":
    uvicorn.run("api:app", host="0.0.0.0", port=8547, reload=False)
