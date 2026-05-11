//! DevilChain REST API
//! ✅ Signature verified on every send_transaction
//! ✅ Types unified: BlockStore, ValidatorRegistry
//! ✅ DAO connected
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use axum::{
    routing::{get, post},
    Router, Json, extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tower_http::cors::{CorsLayer, Any};

use crate::blockchain::{Blockchain, Transaction, Amount};
use crate::mempool::Mempool;
use crate::consensus::ValidatorRegistry;  // ✅ canonical name
use crate::dao::DaoGovernance;
use crate::tokenomics::{DECIMALS, DEV_WALLET, MINING_POOL_WALLET, MIN_GAS_FEE};

// ── Shared state ──────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub mempool:    Arc<RwLock<Mempool>>,
    pub validators: Arc<RwLock<ValidatorRegistry>>,
    pub dao:        Arc<RwLock<DaoGovernance>>,
}

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct SendTxRequest {
    pub from:       String,
    pub to:         String,
    pub amount:     u128,      // µDVC ✅ u128
    pub gas_fee:    u128,      // µDVC
    pub nonce:      u64,
    pub public_key: String,    // hex(32-byte Ed25519 verifying key)
    pub signature:  String,    // hex(64-byte Ed25519 sig over tx hash)
    pub data:       Option<String>,
}

#[derive(Deserialize)]
pub struct StakeRequest {
    pub address:    String,
    pub amount:     u128,
    pub signature:  String,
    pub public_key: String,
}

#[derive(Deserialize)]
pub struct VoteRequest {
    pub address:     String,
    pub proposal_id: u64,
    pub vote:        bool,
    pub signature:   String,
    pub public_key:  String,
}

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub ok:   bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn ok(data: T)     -> (StatusCode, Json<Self>) {
        (StatusCode::OK, Json(Self { ok: true, data: Some(data), error: None }))
    }
    fn err(msg: &str)  -> (StatusCode, Json<Self>) {
        (StatusCode::BAD_REQUEST, Json(Self { ok: false, data: None,
            error: Some(msg.to_string()) }))
    }
}

// ── Server entry point ────────────────────────────────────────────────────────

/// ✅ Correct signature — called from main.rs with these exact args
pub async fn start_api_server(
    blockchain: Arc<RwLock<Blockchain>>,
    mempool:    Arc<RwLock<Mempool>>,
    validators: Arc<RwLock<ValidatorRegistry>>,
    dao:        Arc<RwLock<DaoGovernance>>,
) -> Result<(), String> {
    let port = std::env::var("NODE_PORT")
        .ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(8545);

    let state = AppState { blockchain, mempool, validators, dao };
    let app   = Router::new()
        .route("/api/status",           get(status))
        .route("/api/block/latest",      get(latest_block))
        .route("/api/block/:height",     get(get_block))
        .route("/api/tx/:hash",          get(get_tx))
        .route("/api/wallet/:address",   get(wallet_info))
        .route("/api/validators",        get(get_validators))
        .route("/api/coin",              get(coin_info))
        .route("/api/dao/proposals",     get(dao_proposals))
        .route("/api/send",              post(send_transaction))
        .route("/api/stake",             post(stake))
        .route("/api/vote",              post(vote))
        .route("/api/faucet",            post(faucet))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await.map_err(|e| e.to_string())?;
    log::info!("REST API on 0.0.0.0:{}", port);
    axum::serve(listener, app).await.map_err(|e| e.to_string())
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn status(State(s): State<AppState>)
    -> (StatusCode, Json<ApiResponse<serde_json::Value>>)
{
    let bc = s.blockchain.read().unwrap();
    ApiResponse::ok(serde_json::json!({
        "node": "DevilChain", "version": "1.0.0",
        "height": bc.height(), "chain_id": "devl-testnet-1",
        "circulating": bc.supply.circulating(),
        "dev_wallet": DEV_WALLET,
        "mining_pool": MINING_POOL_WALLET,
    }))
}

async fn latest_block(State(s): State<AppState>)
    -> (StatusCode, Json<ApiResponse<serde_json::Value>>)
{
    let bc = s.blockchain.read().unwrap();
    match bc.latest_block() {
        Some(b) => ApiResponse::ok(serde_json::to_value(b).unwrap_or_default()),
        None    => ApiResponse::err("No blocks"),
    }
}

async fn get_block(
    State(s): State<AppState>, Path(height): Path<u64>
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let bc = s.blockchain.read().unwrap();
    match bc.chain.get(height as usize) {
        Some(b) => ApiResponse::ok(serde_json::to_value(b).unwrap_or_default()),
        None    => ApiResponse::err("Block not found"),
    }
}

/// ✅ O(1) tx lookup via tx_index
async fn get_tx(
    State(s): State<AppState>, Path(hash): Path<String>
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let bc = s.blockchain.read().unwrap();
    match bc.get_transaction(&hash) {
        Some(tx) => ApiResponse::ok(serde_json::to_value(tx).unwrap_or_default()),
        None     => ApiResponse::err("TX not found"),
    }
}

async fn wallet_info(
    State(s): State<AppState>, Path(address): Path<String>
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let bc = s.blockchain.read().unwrap();
    let balance = bc.ledger.balance(&address);
    let nonce   = bc.ledger.nonce(&address);
    let txs: Vec<_> = bc.tx_index.keys()
        .filter_map(|h| bc.get_transaction(h))
        .filter(|tx| tx.from == address || tx.to == address)
        .collect();
    ApiResponse::ok(serde_json::json!({
        "address": address,
        "balance": balance,
        "balance_dvc": balance as f64 / 1_000_000.0,  // display only
        "nonce": nonce,
        "tx_count": txs.len(),
        "transactions": serde_json::to_value(&txs).unwrap_or_default(),
    }))
}

async fn get_validators(State(s): State<AppState>)
    -> (StatusCode, Json<ApiResponse<serde_json::Value>>)
{
    let reg = s.validators.read().unwrap();
    let list: Vec<_> = reg.get_all().iter().map(|v| serde_json::json!({
        "address": v.address, "stake": v.stake,
        "reputation": v.reputation, "blocks": v.blocks_produced, "active": v.active
    })).collect();
    ApiResponse::ok(serde_json::json!({ "validators": list, "count": list.len() }))
}

async fn coin_info(State(s): State<AppState>)
    -> (StatusCode, Json<ApiResponse<serde_json::Value>>)
{
    let bc = s.blockchain.read().unwrap();
    ApiResponse::ok(serde_json::json!({
        "name": "DevilCoin", "symbol": "DVC", "decimals": DECIMALS,
        "max_supply": crate::tokenomics::MAX_SUPPLY,
        "total_minted": bc.supply.total_minted,
        "total_burned": bc.supply.total_burned,
        "circulating": bc.supply.circulating(),
        "locked_liquidity": bc.supply.locked_supply,
        "burn_address": crate::tokenomics::BURN_ADDRESS,
        "dev_wallet": DEV_WALLET,
        "fee_split": { "miner": "60%", "dev": "20%", "burn": "10%", "liquidity": "10%" },
    }))
}

/// ✅ DAO connected — proposals from real DaoGovernance engine
async fn dao_proposals(State(s): State<AppState>)
    -> (StatusCode, Json<ApiResponse<serde_json::Value>>)
{
    let bc  = s.blockchain.read().unwrap();
    let dao = s.dao.read().unwrap();
    let height = bc.height();
    drop(bc);
    let active = dao.get_active(height);
    let list: Vec<_> = active.iter().map(|p| serde_json::json!({
        "id": p.id, "title": p.title, "description": p.description,
        "proposer": p.proposer, "votes_yes": p.votes_yes,
        "votes_no": p.votes_no, "status": format!("{:?}", p.status),
        "expires_at": p.expires_at,
    })).collect();
    ApiResponse::ok(serde_json::json!({ "proposals": list }))
}

/// ✅ CRITICAL: signature verified before any balance change
async fn send_transaction(
    State(s): State<AppState>,
    Json(req): Json<SendTxRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    use sha2::{Sha256, Digest as _};

    // 1. Basic validation
    if req.amount == 0          { return ApiResponse::err("Amount must be > 0"); }
    if req.gas_fee < MIN_GAS_FEE { return ApiResponse::err("Gas fee too low"); }

    // 2. Build a temporary TX to verify signature BEFORE touching state
    let timestamp = crate::wallet::now_secs();
    let tmp_tx = Transaction {
        tx_hash:    String::new(),
        from:       req.from.clone(),
        to:         req.to.clone(),
        amount:     req.amount,
        gas_fee:    req.gas_fee,
        nonce:      req.nonce,
        data:       req.data.clone(),
        signature:  req.signature.clone(),
        public_key: req.public_key.clone(),
        timestamp,
    };

    // ✅ Verify Ed25519 sig — attacker cannot forge from address
    if !tmp_tx.verify_signature() {
        return ApiResponse::err("Invalid signature — transaction rejected");
    }
    // ✅ Verify address ownership
    if tmp_tx.sender_address() != req.from {
        return ApiResponse::err("Public key does not match from address");
    }

    // 3. Nonce check
    let expected_nonce = {
        let bc = s.blockchain.read().unwrap();
        bc.ledger.nonce(&req.from)
    };
    if req.nonce != expected_nonce {
        return ApiResponse::err("Invalid nonce");
    }

    // 4. Balance check
    let cost = req.amount + req.gas_fee;
    {
        let bc = s.blockchain.read().unwrap();
        if bc.ledger.balance(&req.from) < cost {
            return ApiResponse::err("Insufficient balance");
        }
    }

    // 5. Build final TX with hash
    let tx_hash = {
        let payload = format!("{}:{}:{}:{}:{}:{}",
            req.from, req.to, req.amount, req.gas_fee, req.nonce, timestamp);
        hex::encode(Sha256::digest(payload.as_bytes()))
    };
    let tx = Transaction {
        tx_hash: tx_hash.clone(),
        from: req.from, to: req.to, amount: req.amount,
        gas_fee: req.gas_fee, nonce: req.nonce, data: req.data,
        signature: req.signature, public_key: req.public_key, timestamp,
    };

    // 6. Add to mempool
    match s.mempool.write().unwrap().add_transaction(tx) {
        Ok(()) => ApiResponse::ok(serde_json::json!({
            "tx_hash": tx_hash, "status": "pending",
            "message": "Transaction accepted into mempool",
        })),
        Err(e) => ApiResponse::err(e),
    }
}

/// ✅ Stake — signature verified
async fn stake(
    State(s): State<AppState>,
    Json(req): Json<StakeRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    // Verify sig over stake message
    let msg = format!("stake:{}:{}", req.address, req.amount);
    let ok  = verify_sig(&req.public_key, &req.signature, &msg);
    if !ok { return ApiResponse::err("Invalid stake signature"); }

    let mut reg = s.validators.write().unwrap();
    let _ = reg.register(crate::consensus::Validator {
        address:         req.address.clone(),
        stake:           req.amount,
        reputation:      50,
        blocks_produced: 0,
        active:          true,
    });
    ApiResponse::ok(serde_json::json!({ "staked": req.amount, "address": req.address }))
}

/// ✅ Vote — connected to real DAO engine
async fn vote(
    State(s): State<AppState>,
    Json(req): Json<VoteRequest>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let msg = format!("vote:{}:{}", req.proposal_id, req.vote);
    if !verify_sig(&req.public_key, &req.signature, &msg) {
        return ApiResponse::err("Invalid vote signature");
    }
    let voting_power = {
        let reg = s.validators.read().unwrap();
        reg.get(&req.address).map(|v| v.voting_power()).unwrap_or(1_000_000)
    };
    let height = s.blockchain.read().unwrap().height();
    match s.dao.write().unwrap().vote(req.proposal_id, req.address.clone(),
                                      req.vote, voting_power) {
        Ok(()) => ApiResponse::ok(serde_json::json!({
            "voted": req.vote, "proposal_id": req.proposal_id
        })),
        Err(e) => ApiResponse::err(e),
    }
}

async fn faucet(
    State(s): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let addr = match body.get("address").and_then(|v| v.as_str()) {
        Some(a) => a.to_string(),
        None    => return ApiResponse::err("address required"),
    };
    // Faucet: 100 DVC
    let amount = 100 * DECIMALS as u128;
    s.blockchain.write().unwrap().ledger.credit(&addr, amount);
    ApiResponse::ok(serde_json::json!({
        "address": addr, "amount": amount, "message": "100 DVC sent from faucet"
    }))
}

// ── Signature helper ──────────────────────────────────────────────────────────

fn verify_sig(pub_hex: &str, sig_hex: &str, msg: &str) -> bool {
    use ed25519_dalek::{VerifyingKey, Signature, Verifier};
    let pk_b  = match hex::decode(pub_hex)  { Ok(b) if b.len()==32 => b, _ => return false };
    let sig_b = match hex::decode(sig_hex)  { Ok(b) if b.len()==64 => b, _ => return false };
    let arr_pk:  [u8;32] = match pk_b.try_into()  { Ok(a) => a, _ => return false };
    let arr_sig: [u8;64] = match sig_b.try_into() { Ok(a) => a, _ => return false };
    let Ok(vk)  = VerifyingKey::from_bytes(&arr_pk)  else { return false };
    let Ok(sig) = Signature::from_bytes(&arr_sig)    else { return false };
    vk.verify(msg.as_bytes(), &sig).is_ok()
}
