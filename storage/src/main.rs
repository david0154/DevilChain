mod lib;
use lib::DevilStorageNode;
use axum::{routing::{get, post}, Router, extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::info;

#[derive(Deserialize)]
struct StoreReq { data_b64: String, file_name: String, owner: String, is_public: bool }

#[derive(Serialize)]
struct StoreRes { cid: String, size: usize }

type NodeState = Arc<RwLock<DevilStorageNode>>;

async fn store_file(State(node): State<NodeState>, Json(req): Json<StoreReq>) -> Json<serde_json::Value> {
    use base64::{Engine as _, engine::general_purpose};
    let data = general_purpose::STANDARD.decode(&req.data_b64).unwrap_or_default();
    let mut n = node.write().await;
    match n.store(&data, &req.file_name, &req.owner, req.is_public) {
        Ok(cid) => Json(serde_json::json!({ "success": true, "cid": cid, "size": data.len() })),
        Err(e) => Json(serde_json::json!({ "success": false, "error": e.to_string() })),
    }
}

async fn retrieve_file(State(node): State<NodeState>, Path(cid): Path<String>) -> Json<serde_json::Value> {
    use base64::{Engine as _, engine::general_purpose};
    let n = node.read().await;
    match n.retrieve(&cid) {
        Ok(data) => Json(serde_json::json!({ "cid": cid, "data_b64": general_purpose::STANDARD.encode(&data) })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

async fn node_stats(State(node): State<NodeState>) -> Json<serde_json::Value> {
    Json(node.read().await.stats())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    std::fs::create_dir_all("/data/devil-storage").ok();
    let node = Arc::new(RwLock::new(DevilStorageNode::new("db1xstorage001", "/data/devil-storage", 100)));
    let app = Router::new()
        .route("/store", post(store_file))
        .route("/retrieve/:cid", get(retrieve_file))
        .route("/stats", get(node_stats))
        .with_state(node);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8548").await.unwrap();
    info!("DevilStorage Node running on :8548");
    axum::serve(listener, app).await.unwrap();
}
