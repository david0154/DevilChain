//! DevilChain P2P Network — libp2p-inspired HTTP gossip with peer discovery
//! - mDNS-style peer tracking via /api/peers endpoint
//! - Block gossip: push new blocks to all known peers
//! - Chain sync: pull missing blocks from ahead peers
//! - Peer reputation: drop unresponsive peers after 3 failures
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::blockchain::{Blockchain, Block};

#[derive(Debug, Clone)]
struct Peer {
    addr:     String,  // "host:port"
    failures: u8,
    height:   u64,
}

impl Peer {
    fn new(addr: String) -> Self { Self { addr, failures: 0, height: 0 } }
    fn is_alive(&self)   -> bool { self.failures < 3 }
}

pub async fn start_p2p(
    blockchain: Arc<RwLock<Blockchain>>
) -> Result<(), String> {
    let bootstrap: Vec<String> = std::env::var("BOOTSTRAP_PEERS")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    let port = std::env::var("P2P_PORT")
        .ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(30303);

    log::info!("[P2P] Starting on port {} | bootstrap: {:?}", port, bootstrap);

    // Peer table: addr -> Peer
    let mut peers: HashMap<String, Peer> = bootstrap
        .into_iter()
        .map(|a| (a.clone(), Peer::new(a)))
        .collect();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build().map_err(|e| e.to_string())?;

    let mut gossip_tick = tokio::time::interval(std::time::Duration::from_secs(5));
    let mut discovery_tick = tokio::time::interval(std::time::Duration::from_secs(30));

    loop {
        tokio::select! {
            _ = gossip_tick.tick() => {
                let (my_height, my_hash, latest_block_json) = {
                    let bc = blockchain.read().map_err(|e| e.to_string())?;
                    let b  = bc.latest_block();
                    (
                        bc.height(),
                        b.map(|blk| blk.block_hash.clone()).unwrap_or_default(),
                        b.map(|blk| serde_json::to_string(blk).unwrap_or_default()),
                    )
                };

                let dead: Vec<String> = peers.values()
                    .filter(|p| !p.is_alive())
                    .map(|p| p.addr.clone())
                    .collect();
                for d in dead { peers.remove(&d); log::info!("[P2P] Dropped dead peer {}", d); }

                for peer in peers.values_mut() {
                    // 1. Fetch peer status
                    let url = format!("http://{}/api/status", peer.addr);
                    let status_res = client.get(&url).send().await;
                    let status = match status_res {
                        Ok(r) => match r.json::<serde_json::Value>().await {
                            Ok(v) => v,
                            Err(_) => { peer.failures += 1; continue; }
                        },
                        Err(_) => { peer.failures += 1; continue; }
                    };
                    peer.failures = 0;  // responsive — reset
                    peer.height = status
                        .get("data").and_then(|d| d.get("height"))
                        .and_then(|h| h.as_u64()).unwrap_or(0);

                    // 2. If peer is behind us — push our latest block (gossip)
                    if peer.height < my_height {
                        if let Some(ref bj) = latest_block_json {
                            let push_url = format!("http://{}/api/p2p/block", peer.addr);
                            let _ = client.post(&push_url)
                                .header("Content-Type", "application/json")
                                .body(bj.clone())
                                .send().await;
                            log::debug!("[P2P] Pushed block #{} to {}", my_height - 1, peer.addr);
                        }
                    }

                    // 3. If peer is ahead of us — pull missing blocks
                    if peer.height > my_height {
                        log::info!("[P2P] Peer {} ahead ({} > {}) — syncing",
                            peer.addr, peer.height, my_height);
                        for h in my_height..peer.height {
                            let block_url = format!("http://{}/api/block/{}", peer.addr, h);
                            if let Ok(resp) = client.get(&block_url).send().await {
                                if let Ok(body) = resp.json::<serde_json::Value>().await {
                                    if let Some(bv) = body.get("data") {
                                        if let Ok(block) = serde_json::from_value::<Block>(bv.clone()) {
                                            let mut bc = blockchain.write()
                                                .map_err(|e| e.to_string())?;
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

                    log::debug!("[P2P] Peer {} @ height {}", peer.addr, peer.height);
                }
            }

            _ = discovery_tick.tick() => {
                // Peer discovery: ask each known peer for their peer list
                let addrs: Vec<String> = peers.keys().cloned().collect();
                for addr in addrs {
                    let url = format!("http://{}/api/peers", addr);
                    if let Ok(resp) = client.get(&url).send().await {
                        if let Ok(body) = resp.json::<serde_json::Value>().await {
                            if let Some(arr) = body.get("peers").and_then(|v| v.as_array()) {
                                for p in arr {
                                    if let Some(s) = p.as_str() {
                                        peers.entry(s.to_string())
                                            .or_insert_with(|| {
                                                log::info!("[P2P] Discovered new peer: {}", s);
                                                Peer::new(s.to_string())
                                            });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
