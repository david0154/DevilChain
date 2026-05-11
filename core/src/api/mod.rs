//! DevilChain REST API — Axum

use axum::{
    routing::get,
    Router, Json,
    extract::{Path, State},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::blockchain::Blockchain;

pub async fn start_api_server(blockchain: Arc<RwLock<Blockchain>>) {
    let app = Router::new()
        .route("/api/block/latest", get(get_latest_block))
        .route("/api/block/:height", get(get_block_by_height))
        .route("/api/status", get(get_status))
        .with_state(blockchain);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8545").await.unwrap();
    log::info!("DevilChain API listening on :8545");
    axum::serve(listener, app).await.unwrap();
}

async fn get_latest_block(
    State(bc): State<Arc<RwLock<Blockchain>>>,
) -> Json<serde_json::Value> {
    let chain = bc.read().await;
    let block = chain.latest_block();
    Json(serde_json::to_value(block).unwrap())
}

async fn get_block_by_height(
    State(bc): State<Arc<RwLock<Blockchain>>>,
    Path(height): Path<u64>,
) -> Json<serde_json::Value> {
    let chain = bc.read().await;
    let block = chain.chain.iter().find(|b| b.block_height == height);
    match block {
        Some(b) => Json(serde_json::to_value(b).unwrap()),
        None => Json(serde_json::json!({"error": "Block not found"})),
    }
}

async fn get_status(
    State(bc): State<Arc<RwLock<Blockchain>>>,
) -> Json<serde_json::Value> {
    let chain = bc.read().await;
    Json(serde_json::json!({
        "network": "DevilChain",
        "coin": "DevilCoin (DVC)",
        "symbol": "DVL",
        "chain_length": chain.chain.len(),
        "latest_height": chain.latest_block().block_height
    }))
}
