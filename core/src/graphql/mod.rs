//! DevilChain GraphQL API
//! ✅ O(1) transaction lookup via tx_index (no full chain scan)
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use async_graphql::{
    Schema, Object, SimpleObject, EmptyMutation, EmptySubscription, Context,
    http::{GraphiQLSource, playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    routing::get, Router, response::Html,
    extract::State,
};
use std::sync::{Arc, RwLock};
use crate::blockchain::Blockchain;

#[derive(SimpleObject, Clone)]
struct GqlBlock {
    height:        i64,
    block_hash:    String,
    previous_hash: String,
    timestamp:     i64,
    validator:     String,
    tx_count:      i32,
    block_reward:  String,
    total_fees:    String,
    ai_score:      i32,
}

#[derive(SimpleObject, Clone)]
struct GqlTx {
    tx_hash:   String,
    from:      String,
    to:        String,
    amount:    String,   // µDVC as string (avoids i64 overflow)
    gas_fee:   String,
    nonce:     i64,
    timestamp: i64,
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn status(&self, ctx: &Context<'_>) -> String {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>().unwrap().read().unwrap();
        format!("height={} circulating={}", bc.height(), bc.supply.circulating())
    }

    async fn block(&self, ctx: &Context<'_>, height: i64) -> Option<GqlBlock> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>().unwrap().read().unwrap();
        bc.chain.get(height as usize).map(|b| GqlBlock {
            height:        b.height as i64,
            block_hash:    b.block_hash.clone(),
            previous_hash: b.previous_hash.clone(),
            timestamp:     b.timestamp as i64,
            validator:     b.validator.clone(),
            tx_count:      b.transactions.len() as i32,
            block_reward:  b.block_reward.to_string(),
            total_fees:    b.total_fees.to_string(),
            ai_score:      b.ai_score as i32,
        })
    }

    async fn latest_block(&self, ctx: &Context<'_>) -> Option<GqlBlock> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>().unwrap().read().unwrap();
        bc.latest_block().map(|b| GqlBlock {
            height:        b.height as i64,
            block_hash:    b.block_hash.clone(),
            previous_hash: b.previous_hash.clone(),
            timestamp:     b.timestamp as i64,
            validator:     b.validator.clone(),
            tx_count:      b.transactions.len() as i32,
            block_reward:  b.block_reward.to_string(),
            total_fees:    b.total_fees.to_string(),
            ai_score:      b.ai_score as i32,
        })
    }

    /// ✅ O(1) — uses tx_index HashMap, no chain scan
    async fn transaction(&self, ctx: &Context<'_>, hash: String) -> Option<GqlTx> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>().unwrap().read().unwrap();
        bc.get_transaction(&hash).map(|tx| GqlTx {
            tx_hash:   tx.tx_hash.clone(),
            from:      tx.from.clone(),
            to:        tx.to.clone(),
            amount:    tx.amount.to_string(),
            gas_fee:   tx.gas_fee.to_string(),
            nonce:     tx.nonce as i64,
            timestamp: tx.timestamp as i64,
        })
    }

    async fn wallet(
        &self, ctx: &Context<'_>, address: String
    ) -> async_graphql::Json<serde_json::Value> {
        let bc = ctx.data::<Arc<RwLock<Blockchain>>>().unwrap().read().unwrap();
        let balance = bc.ledger.balance(&address);
        let nonce   = bc.ledger.nonce(&address);
        async_graphql::Json(serde_json::json!({
            "address": address, "balance": balance.to_string(), "nonce": nonce
        }))
    }
}

pub type DevilSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn start_graphql_server(
    blockchain: Arc<RwLock<Blockchain>>
) -> Result<(), String> {
    let port = std::env::var("GRAPHQL_PORT")
        .ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(8546);

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(blockchain)
        .finish();

    let app = Router::new()
        .route("/graphql",   get(graphiql).post(graphql_handler))
        .with_state(schema);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await.map_err(|e| e.to_string())?;
    log::info!("GraphQL on 0.0.0.0:{}", port);
    axum::serve(listener, app).await.map_err(|e| e.to_string())
}

async fn graphiql() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    State(schema): State<DevilSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
