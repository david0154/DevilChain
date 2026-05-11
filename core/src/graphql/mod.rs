//! DevilChain GraphQL API
//! Runs on port 8546 alongside REST (8545)

use async_graphql::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::blockchain::Blockchain;

// --- GraphQL Types ---

#[derive(SimpleObject, Clone)]
pub struct GqlTransaction {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub gas_fee: f64,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(SimpleObject, Clone)]
pub struct GqlBlock {
    pub block_height: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub validator: String,
    pub merkle_root: String,
    pub nonce: u64,
    pub ai_score: f64,
    pub dao_signature: String,
    pub block_hash: String,
    pub transactions: Vec<GqlTransaction>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get the latest block
    async fn latest_block(&self, ctx: &Context<'_>) -> Result<GqlBlock> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?;
        let chain = bc.read().await;
        let b = chain.latest_block();
        Ok(to_gql_block(b))
    }

    /// Get block by height
    async fn block(&self, ctx: &Context<'_>, height: u64) -> Result<Option<GqlBlock>> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?;
        let chain = bc.read().await;
        Ok(chain.chain.iter().find(|b| b.block_height == height).map(to_gql_block))
    }

    /// Get network status
    async fn status(&self, ctx: &Context<'_>) -> Result<String> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?;
        let chain = bc.read().await;
        Ok(format!("DevilChain | Height: {} | TPS: 5000-20000", chain.latest_block().block_height))
    }

    /// Get all blocks (paginated)
    async fn blocks(&self, ctx: &Context<'_>, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<GqlBlock>> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>()?;
        let chain = bc.read().await;
        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(10) as usize;
        let blocks = chain.chain.iter().rev().skip(offset).take(limit).map(to_gql_block).collect();
        Ok(blocks)
    }
}

fn to_gql_block(b: &crate::blockchain::Block) -> GqlBlock {
    GqlBlock {
        block_height: b.block_height,
        timestamp: b.timestamp,
        previous_hash: b.previous_hash.clone(),
        validator: b.validator.clone(),
        merkle_root: b.merkle_root.clone(),
        nonce: b.nonce,
        ai_score: b.ai_score,
        dao_signature: b.dao_signature.clone(),
        block_hash: b.block_hash.clone(),
        transactions: b.transactions.iter().map(|tx| GqlTransaction {
            tx_hash: tx.tx_hash.clone(),
            from: tx.from.clone(),
            to: tx.to.clone(),
            amount: tx.amount,
            gas_fee: tx.gas_fee,
            timestamp: tx.timestamp,
            signature: tx.signature.clone(),
        }).collect(),
    }
}

pub type DevilSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(blockchain: Arc<RwLock<Blockchain>>) -> DevilSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(blockchain)
        .finish()
}

pub async fn start_graphql_server(blockchain: Arc<RwLock<Blockchain>>) {
    use async_graphql_axum::{GraphQL, GraphQLSubscription};
    use axum::{Router, routing::get};

    let schema = build_schema(blockchain);
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post_service(GraphQL::new(schema.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8546").await.unwrap();
    log::info!("GraphQL Playground: http://localhost:8546/graphql");
    axum::serve(listener, app).await.unwrap();
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}
