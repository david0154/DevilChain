//! DevilChain P2P — HTTP-based block gossip between nodes
//! Peers exchange block headers; lagging peers request missing blocks.
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::sync::{Arc, RwLock};
use crate::blockchain::{Blockchain, Block};

pub async fn start_p2p(
    blockchain: Arc<RwLock<Blockchain>>
) -> Result<(), String> {
    let port = std::env::var("P2P_PORT")
        .ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(30303);
    let peers: Vec<String> = std::env::var("BOOTSTRAP_PEERS")
        .unwrap_or_default().split(',')
        .filter(|s| !s.is_empty())
        .map(String::from).collect();

    log::info!("P2P listening on port {} | peers: {:?}", port, peers);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(4))
        .build().map_err(|e| e.to_string())?;

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;

        let (my_height, my_hash) = {
            let bc = blockchain.read().map_err(|e| e.to_string())?;
            (bc.height(), bc.latest_block().map(|b| b.block_hash.clone()).unwrap_or_default())
        };

        for peer in &peers {
            // 1. Query peer status
            let peer_status = match client
                .get(&format!("http://{}/api/status", peer)).send().await
            {
                Ok(r)  => match r.json::<serde_json::Value>().await {
                    Ok(v) => v,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            let peer_height = peer_status
                .get("data").and_then(|d| d.get("height"))
                .and_then(|h| h.as_u64()).unwrap_or(0);

            // 2. If peer is ahead, fetch missing blocks
            if peer_height > my_height {
                log::info!("[P2P] Peer {} is ahead ({} > {}) — syncing",
                    peer, peer_height, my_height);
                for h in my_height..peer_height {
                    let url = format!("http://{}/api/block/{}", peer, h);
                    if let Ok(resp) = client.get(&url).send().await {
                        if let Ok(body) = resp.json::<serde_json::Value>().await {
                            if let Some(block_val) = body.get("data") {
                                if let Ok(block) = serde_json::from_value::<Block>(
                                    block_val.clone()
                                ) {
                                    let mut bc = blockchain.write().map_err(|e| e.to_string())?;
                                    match bc.add_block(block) {
                                        Ok(())  => log::info!("[P2P] Synced block #{}", h),
                                        Err(e)  => log::warn!("[P2P] Block #{} rejected: {}", h, e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        log::debug!("[P2P] height={} hash={:.8}", my_height, my_hash);
    }
}
