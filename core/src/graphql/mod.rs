//! DevilChain GraphQL API — async-graphql
//! Runs on port 8546 alongside REST (8545)

use async_graphql::{Object, Schema, EmptyMutation, EmptySubscription, SimpleObject};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, routing::post, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::blockchain::Blockchain;

#[derive(SimpleObject, Clone)]
pub struct BlockGQL {
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
pub struct TransactionGQL {
    pub tx_hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub gas_fee: f64,
    pub timestamp: i64,
}

#[derive(SimpleObject, Clone)]
pub struct NetworkStatus {
    pub network: String,
    pub coin: String,
    pub symbol: String,
    pub chain_length: i64,
    pub latest_height: i64,
    pub tps_target: String,
    pub block_time: String,
    pub consensus: String,
}

pub struct QueryRoot {
    pub blockchain: Arc<RwLock<Blockchain>>,
}

#[Object]
impl QueryRoot {
    async fn latest_block(&self) -> BlockGQL {
        let bc = self.blockchain.read().await;
        let b = bc.latest_block();
        BlockGQL {
            height: b.block_height as i64,
            hash: b.block_hash.clone(),
            timestamp: b.timestamp as i64,
            validator: b.validator.clone(),
            merkle_root: b.merkle_root.clone(),
            nonce: b.nonce as i64,
            ai_score: b.ai_score,
            dao_signature: b.dao_signature.clone(),
            tx_count: b.transactions.len() as i32,
        }
    }

    async fn block(&self, height: i64) -> Option<BlockGQL> {
        let bc = self.blockchain.read().await;
        bc.chain.iter().find(|b| b.block_height == height as u64).map(|b| BlockGQL {
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

    async fn status(&self) -> NetworkStatus {
        let bc = self.blockchain.read().await;
        NetworkStatus {
            network: "DevilChain".to_string(),
            coin: "DevilCoin (DVC)".to_string(),
            symbol: "DVL".to_string(),
            chain_length: bc.chain.len() as i64,
            latest_height: bc.latest_block().block_height as i64,
            tps_target: "5000-20000".to_string(),
            block_time: "2-5s".to_string(),
            consensus: "Devil Hybrid Protocol (DHP)".to_string(),
        }
    }

    async fn transaction(&self, tx_hash: String) -> Option<TransactionGQL> {
        let bc = self.blockchain.read().await;
        for block in &bc.chain {
            for tx in &block.transactions {
                if tx.tx_hash == tx_hash {
                    return Some(TransactionGQL {
                        tx_hash: tx.tx_hash.clone(),
                        from: tx.from.clone(),
                        to: tx.to.clone(),
                        amount: tx.amount,
                        gas_fee: tx.gas_fee,
                        timestamp: tx.timestamp as i64,
                    });
                }
            }
        }
        None
    }
}

pub type DevilSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(blockchain: Arc<RwLock<Blockchain>>) -> DevilSchema {
    Schema::build(QueryRoot { blockchain }, EmptyMutation, EmptySubscription).finish()
}

pub async fn graphql_handler(
    State(schema): State<DevilSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn start_graphql_server(blockchain: Arc<RwLock<Blockchain>>) {
    let schema = build_schema(blockchain);
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .with_state(schema);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8546").await.unwrap();
    log::info!("GraphQL API listening on :8546/graphql");
    axum::serve(listener, app).await.unwrap();
}
