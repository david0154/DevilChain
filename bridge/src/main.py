"""
DevilBridge — Cross-Chain Bridge Relayer
Supports: DevilChain ↔ Ethereum ↔ BSC ↔ Polygon ↔ Solana
Powered by Devil One (https://devilone.in) | Developed by Nexuzy Lab (https://nexuzy.tech)
"""

import os, hashlib, secrets, time
from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional
import uvicorn

app = FastAPI(
    title="DevilBridge",
    description="DevilChain Cross-Chain Bridge Relayer",
    version="1.0.0"
)

app.add_middleware(CORSMiddleware,
    allow_origins=["*"], allow_methods=["*"], allow_headers=["*"])

# ── Supported chains ──────────────────────────────────────────────────
CHAINS = {
    "devilchain": {"id": "devl-testnet-1",   "symbol": "DVC",  "fee": 0.01},
    "ethereum":   {"id": "1",                 "symbol": "ETH",  "fee": 0.005},
    "bsc":        {"id": "56",                "symbol": "BNB",  "fee": 0.003},
    "polygon":    {"id": "137",               "symbol": "MATIC","fee": 0.002},
    "solana":     {"id": "mainnet-beta",      "symbol": "SOL",  "fee": 0.001},
}

FEE_PCT   = 0.001   # 0.1% bridge fee
bridgeTxs = {}      # In-memory store (replace with DB in production)

# ── Models ────────────────────────────────────────────────────────────
class BridgeRequest(BaseModel):
    from_chain:  str
    to_chain:    str
    from_addr:   str
    to_addr:     str
    amount:      float
    token:       str = "DVC"
    signature:   Optional[str] = None

class LockRequest(BaseModel):
    chain:     str
    address:   str
    amount:    float
    tx_hash:   str

class MintRequest(BaseModel):
    bridge_id: str
    to_addr:   str
    amount:    float
    chain:     str

# ── Routes ────────────────────────────────────────────────────────────
@app.get("/health")
def health():
    return {"status": "ok", "service": "DevilBridge",
            "chains": list(CHAINS.keys()), "version": "1.0.0"}

@app.get("/chains")
def get_chains():
    return {"chains": CHAINS}

@app.post("/bridge/initiate")
def initiate_bridge(req: BridgeRequest):
    if req.from_chain not in CHAINS:
        raise HTTPException(400, f"Unsupported chain: {req.from_chain}")
    if req.to_chain not in CHAINS:
        raise HTTPException(400, f"Unsupported chain: {req.to_chain}")
    if req.amount <= 0:
        raise HTTPException(400, "Amount must be > 0")

    bridge_fee = round(req.amount * FEE_PCT, 6)
    net_amount = round(req.amount - bridge_fee, 6)
    bridge_id  = f"dvlbridge_{secrets.token_hex(16)}"

    bridgeTxs[bridge_id] = {
        "bridge_id":   bridge_id,
        "from_chain":  req.from_chain,
        "to_chain":    req.to_chain,
        "from_addr":   req.from_addr,
        "to_addr":     req.to_addr,
        "amount":      req.amount,
        "bridge_fee":  bridge_fee,
        "net_amount":  net_amount,
        "token":       req.token,
        "status":      "pending_lock",
        "created_at":  int(time.time()),
        "lock_tx":     None,
        "mint_tx":     None,
    }

    return {
        "bridge_id":    bridge_id,
        "status":       "pending_lock",
        "amount":       req.amount,
        "bridge_fee":   bridge_fee,
        "net_amount":   net_amount,
        "lock_address": f"dvlbridge_vault_{req.from_chain}",
        "instructions": f"Send {req.amount} {req.token} to the lock address on {req.from_chain}",
    }

@app.post("/bridge/lock")
def confirm_lock(req: LockRequest):
    # Find pending bridge TX by chain + address
    matching = [b for b in bridgeTxs.values()
                if b["from_chain"] == req.chain and b["from_addr"] == req.address
                and b["status"] == "pending_lock"]
    if not matching:
        raise HTTPException(404, "No pending bridge TX found for this address")

    btx = matching[0]
    btx["status"]  = "locked"
    btx["lock_tx"] = req.tx_hash

    # Auto-generate mint TX hash (relayer would watch the vault in production)
    mint_hash = f"dvlmint_{hashlib.sha256((btx['bridge_id'] + req.tx_hash).encode()).hexdigest()[:32]}"
    btx["mint_tx"] = mint_hash
    btx["status"]  = "minted"
    btx["minted_at"] = int(time.time())

    return {
        "bridge_id":   btx["bridge_id"],
        "status":      "minted",
        "lock_tx":     req.tx_hash,
        "mint_tx":     mint_hash,
        "net_amount":  btx["net_amount"],
        "to_chain":    btx["to_chain"],
        "to_addr":     btx["to_addr"],
    }

@app.get("/bridge/status/{bridge_id}")
def bridge_status(bridge_id: str):
    if bridge_id not in bridgeTxs:
        raise HTTPException(404, "Bridge TX not found")
    return bridgeTxs[bridge_id]

@app.get("/bridge/history/{address}")
def bridge_history(address: str):
    txs = [b for b in bridgeTxs.values()
           if b["from_addr"] == address or b["to_addr"] == address]
    return {"address": address, "bridges": txs, "count": len(txs)}

@app.get("/stats")
def bridge_stats():
    total    = len(bridgeTxs)
    minted   = sum(1 for b in bridgeTxs.values() if b["status"] == "minted")
    pending  = sum(1 for b in bridgeTxs.values() if b["status"] == "pending_lock")
    vol      = sum(b["amount"] for b in bridgeTxs.values() if b["status"] == "minted")
    return {
        "total_bridges":  total,
        "minted":         minted,
        "pending":        pending,
        "total_volume":   round(vol, 4),
        "supported_chains": list(CHAINS.keys()),
        "fee_percent":    f"{FEE_PCT * 100}%",
    }

if __name__ == "__main__":
    port = int(os.getenv("PORT", 8549))
    uvicorn.run("main:app", host="0.0.0.0", port=port, reload=True)
