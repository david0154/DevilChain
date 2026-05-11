//! DevilChain GraphQL API — async-graphql
//! Serves on port 8546

use async_graphql::{Schema, Object, Context, SimpleObject, Result as GqlResult};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{Router, routing::get};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::blockchain::Blockchain;

#[derive(SimpleObject, Clone)]
pub struct GqlBlock {
    pub height: i64,
    pub hash: String,
    pub timestamp: i64,
    pub validator: String,
    pub merkle_root: String,
    pub nonce: i64,
    pub ai_score: f64,
    pub dao_signature: String,
    pub tx_count: i32,
}

#[derive(SimpleObject, Clone)]
pub struct GqlTransaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub gas_fee: f64,
    pub timestamp: i64,
}

#[derive(SimpleObject, Clone)]
pub struct GqlNetworkStatus {
    pub network: String,
    pub coin: String,
    pub symbol: String,
    pub chain_length: i64,
    pub latest_height: i64,
    pub tps_target: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn status(&self, ctx: &Context<'_>) -> GqlResult<GqlNetworkStatus> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?.read().await;
        Ok(GqlNetworkStatus {
            network: "DevilChain".to_string(),
            coin: "DevilCoin (DVC)".to_string(),
            symbol: "DVL".to_string(),
            chain_length: bc.chain.len() as i64,
            latest_height: bc.latest_block().block_height as i64,
            tps_target: "5000-20000".to_string(),
        })
    }

    async fn latest_block(&self, ctx: &Context<'_>) -> GqlResult<GqlBlock> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?.read().await;
        let b = bc.latest_block();
        Ok(GqlBlock {
            height: b.block_height as i64,
            hash: b.block_hash.clone(),
            timestamp: b.timestamp as i64,
            validator: b.validator.clone(),
            merkle_root: b.merkle_root.clone(),
            nonce: b.nonce as i64,
            ai_score: b.ai_score,
            dao_signature: b.dao_signature.clone(),
            tx_count: b.transactions.len() as i32,
        })
    }

    async fn block(&self, ctx: &Context<'_>, height: i64) -> GqlResult<Option<GqlBlock>> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?.read().await;
        Ok(bc.chain.iter().find(|b| b.block_height == height as u64).map(|b| GqlBlock {
            height: b.block_height as i64,
            hash: b.block_hash.clone(),
            timestamp: b.timestamp as i64,
            validator: b.validator.clone(),
            merkle_root: b.merkle_root.clone(),
            nonce: b.nonce as i64,
            ai_score: b.ai_score,
            dao_signature: b.dao_signature.clone(),
            tx_count: b.transactions.len() as i32,
        }))
    }

    async fn transactions(&self, ctx: &Context<'_>, limit: Option<i32>) -> GqlResult<Vec<GqlTransaction>> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?.read().await;
        let limit = limit.unwrap_or(20) as usize;
        let mut txs = vec![];
        for block in bc.chain.iter().rev() {
            for tx in &block.transactions {
                txs.push(GqlTransaction {
                    tx_hash: tx.tx_hash.clone(),
                    from: tx.from.clone(),
                    to: tx.to.clone(),
                    amount: tx.amount,
                    gas_fee: tx.gas_fee,
                    timestamp: tx.timestamp as i64,
                });
                if txs.len() >= limit { break; }
            }
            if txs.len() >= limit { break; }
        }
        Ok(txs)
    }
}

pub type DevilSchema = Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn build_schema(blockchain: Arc<RwLock<Blockchain>>) -> DevilSchema {
    Schema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(blockchain)
        .finish()
}

pub async fn start_graphql_server(blockchain: Arc<RwLock<Blockchain>>) {
    let schema = build_schema(blockchain);
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post_service(GraphQL::new(schema.clone())))
        .route("/graphql/ws", get(GraphQLSubscription::new(schema)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8546").await.unwrap();
    log::info!("DevilChain GraphQL listening on :8546/graphql");
    axum::serve(listener, app).await.unwrap();
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
    ))
}
