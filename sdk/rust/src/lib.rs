//! DevilChain Rust SDK
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub network: String,
    pub coin: String,
    pub symbol: String,
    pub chain_length: u64,
    pub latest_height: u64,
}

pub struct DevilChainClient { pub base_url: String }

impl DevilChainClient {
    pub fn new(base_url: &str) -> Self { DevilChainClient { base_url: base_url.to_string() } }
    pub async fn get_status(&self) -> anyhow::Result<NetworkStatus> {
        let url = format!("{}/api/status", self.base_url);
        Ok(reqwest::get(&url).await?.json::<NetworkStatus>().await?)
    }
    pub async fn get_latest_block(&self) -> anyhow::Result<serde_json::Value> {
        Ok(reqwest::get(format!("{}/api/block/latest", self.base_url)).await?.json().await?)
    }
    pub async fn get_wallet(&self, addr: &str) -> anyhow::Result<serde_json::Value> {
        Ok(reqwest::get(format!("{}/api/wallet/{}", self.base_url, addr)).await?.json().await?)
    }
}
