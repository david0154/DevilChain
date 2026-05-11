//! DevilChain REST API — Axum (port 8545)

use axum::{
    routing::{get, post},
    Router, Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::blockchain::{Blockchain, Transaction, now_timestamp};
use crate::validator::ValidatorSet;
use crate::storage::ChainDB;

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub validators: Arc<RwLock<ValidatorSet>>,
    pub db: Arc<ChainDB>,
}

#[derive(Deserialize)]
pub struct SendTxRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub gas_fee: f64,
    pub signature: String,
}

#[derive(Deserialize)]
pub struct StakeRequest {
    pub address: String,
    pub amount: f64,
    pub signature: String,
}

#[derive(Deserialize)]
pub struct VoteRequest {
    pub proposal_id: u64,
    pub voter: String,
    pub in_favor: bool,
    pub signature: String,
}

pub async fn start_api_server(
    blockchain: Arc<RwLock<Blockchain>>,
    validators: Arc<RwLock<ValidatorSet>>,
    db: Arc<ChainDB>,
) {
    let state = AppState { blockchain, validators, db };

    let app = Router::new()
        .route("/api/status",            get(get_status))
        .route("/api/block/latest",       get(get_latest_block))
        .route("/api/block/:height",      get(get_block_by_height))
        .route("/api/tx/:hash",           get(get_tx))
        .route("/api/wallet/:address",    get(get_wallet))
        .route("/api/validators",         get(get_validators))
        .route("/api/dao/proposals",      get(get_dao_proposals))
        .route("/api/send",               post(send_transaction))
        .route("/api/stake",              post(stake))
        .route("/api/unstake",            post(unstake))
        .route("/api/vote",               post(vote))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8545").await.unwrap();
    log::info!("DevilChain REST API listening on :8545");
    axum::serve(listener, app).await.unwrap();
}

async fn get_status(State(s): State<AppState>) -> Json<serde_json::Value> {
    let chain = s.blockchain.read().await;
    let vals  = s.validators.read().await;
    Json(serde_json::json!({
        "network":        "DevilChain",
        "coin":           "DevilCoin (DVC)",
        "symbol":         "DVL",
        "address_prefix": "db1x",
        "chain_length":   chain.chain.len(),
        "latest_height":  chain.latest_block().block_height,
        "active_validators": vals.active_validators().len(),
        "total_staked":   vals.total_staked(),
        "tps_target":     "5000-20000",
        "consensus":      "Devil Hybrid Protocol (DHP)"
    }))
}

async fn get_latest_block(State(s): State<AppState>) -> Json<serde_json::Value> {
    let chain = s.blockchain.read().await;
    Json(serde_json::to_value(chain.latest_block()).unwrap())
}

async fn get_block_by_height(State(s): State<AppState>, Path(height): Path<u64>) -> Json<serde_json::Value> {
    let chain = s.blockchain.read().await;
    match chain.chain.iter().find(|b| b.block_height == height) {
        Some(b) => Json(serde_json::to_value(b).unwrap()),
        None => Json(serde_json::json!({"error": "Block not found", "height": height})),
    }
}

async fn get_tx(State(s): State<AppState>, Path(hash): Path<String>) -> Json<serde_json::Value> {
    match s.db.get_tx(&hash) {
        Ok(Some(tx)) => Json(serde_json::to_value(tx).unwrap()),
        _ => Json(serde_json::json!({"error": "Transaction not found", "hash": hash})),
    }
}

async fn get_wallet(State(s): State<AppState>, Path(address): Path<String>) -> Json<serde_json::Value> {
    let balance = s.db.get_balance(&address).unwrap_or(0.0);
    Json(serde_json::json!({
        "address": address,
        "balance": balance,
        "coin": "DVC"
    }))
}

async fn get_validators(State(s): State<AppState>) -> Json<serde_json::Value> {
    let vals = s.validators.read().await;
    let list: Vec<serde_json::Value> = vals.active_validators().iter().map(|v| serde_json::json!({
        "address":          v.address,
        "staked":           v.staked_amount,
        "reputation":       v.reputation_score,
        "voting_power":     v.voting_power(),
        "blocks_validated": v.blocks_validated,
        "blocks_missed":    v.blocks_missed,
        "is_active":        v.is_active,
    })).collect();
    Json(serde_json::json!({ "validators": list, "total": list.len() }))
}

async fn get_dao_proposals(State(s): State<AppState>) -> Json<serde_json::Value> {
    // TODO: connect to DAO engine store; returning placeholder
    Json(serde_json::json!({ "proposals": [], "message": "DAO proposals endpoint" }))
}

async fn send_transaction(State(s): State<AppState>, Json(req): Json<SendTxRequest>) -> (StatusCode, Json<serde_json::Value>) {
    if !crate::wallet::Wallet::is_valid_address(&req.from) || !crate::wallet::Wallet::is_valid_address(&req.to) {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Invalid address format"})));
    }
    if req.amount <= 0.0 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Amount must be positive"})));
    }
    if req.gas_fee < 0.001 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Gas fee too low, minimum 0.001 DVC"})));
    }
    // Check sender balance
    let balance = s.db.get_balance(&req.from).unwrap_or(0.0);
    if balance < req.amount + req.gas_fee {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Insufficient balance"})));
    }
    let tx = Transaction {
        tx_hash: format!("0x{:x}", sha2::Sha256::digest(format!("{}{}{}", req.from, req.to, now_timestamp()).as_bytes())),
        from: req.from.clone(),
        to: req.to.clone(),
        amount: req.amount,
        gas_fee: req.gas_fee,
        timestamp: now_timestamp(),
        signature: req.signature.clone(),
    };
    // Debit sender, credit receiver
    let _ = s.db.update_balance(&req.from, -(req.amount + req.gas_fee));
    let _ = s.db.update_balance(&req.to, req.amount);
    let _ = s.db.put_tx(&tx);
    (StatusCode::OK, Json(serde_json::json!({ "success": true, "tx_hash": tx.tx_hash })))
}

async fn stake(State(s): State<AppState>, Json(req): Json<StakeRequest>) -> (StatusCode, Json<serde_json::Value>) {
    let balance = s.db.get_balance(&req.address).unwrap_or(0.0);
    if balance < req.amount {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "Insufficient balance to stake"})));
    }
    let mut vals = s.validators.write().await;
    match vals.register(req.address.clone(), req.amount, now_timestamp()) {
        Ok(_) => {
            let _ = s.db.update_balance(&req.address, -req.amount);
            (StatusCode::OK, Json(serde_json::json!({ "success": true, "staked": req.amount, "address": req.address })))
        },
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e }))),
    }
}

async fn unstake(State(s): State<AppState>, Json(req): Json<StakeRequest>) -> (StatusCode, Json<serde_json::Value>) {
    let mut vals = s.validators.write().await;
    vals.unregister(&req.address);
    let _ = s.db.update_balance(&req.address, req.amount);
    (StatusCode::OK, Json(serde_json::json!({ "success": true, "unstaked": req.amount })))
}

async fn vote(State(s): State<AppState>, Json(req): Json<VoteRequest>) -> (StatusCode, Json<serde_json::Value>) {
    let vals = s.validators.read().await;
    let power = vals.validators.get(&req.voter).map(|v| v.voting_power()).unwrap_or(1.0);
    drop(vals);
    // TODO: connect to full DAO engine
    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "proposal_id": req.proposal_id,
        "voter": req.voter,
        "voting_power": power,
        "in_favor": req.in_favor
    })))
}
