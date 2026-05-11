//! DevilBridge - Cross-chain bridge
//! Supported: Ethereum, BNB Chain, Polygon, Solana

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Chain { DevilChain, Ethereum, BNBChain, Polygon, Solana }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRequest {
    pub from_chain: Chain,
    pub to_chain: Chain,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeStatus { Pending, Locked, Minting, Completed, Failed }

pub struct DevilBridge;

impl DevilBridge {
    pub fn lock(req: &BridgeRequest) -> BridgeStatus {
        log::info!("[Bridge] Locking {} {} from {:?}", req.amount, req.token, req.from_chain);
        BridgeStatus::Locked
    }
    pub fn mint(req: &BridgeRequest) -> BridgeStatus {
        log::info!("[Bridge] Minting {} on {:?} for {}", req.amount, req.to_chain, req.to_address);
        BridgeStatus::Completed
    }
    pub fn bridge(req: BridgeRequest) -> BridgeStatus {
        match Self::lock(&req) {
            BridgeStatus::Locked => Self::mint(&req),
            _ => BridgeStatus::Failed,
        }
    }
}
