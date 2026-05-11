//! DevilChain Block Storage — RocksDB-backed persistence
//! Blocks are NOT just in RAM — serialized to disk
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use crate::blockchain::Block;
use std::path::Path;

/// Minimal persistence layer using sled (embedded DB, no extra daemon needed)
pub struct BlockStore {
    db: sled::Db,
}

impl BlockStore {
    pub fn open(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn save_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let key   = format!("block:{}", block.height);
        let value = serde_json::to_vec(block)?;
        self.db.insert(key.as_bytes(), value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn load_block(&self, height: u64) -> Option<Block> {
        let key = format!("block:{}", height);
        self.db.get(key.as_bytes()).ok()??
            .as_ref()
            .and_then(|v| serde_json::from_slice(v).ok())
    }

    pub fn latest_height(&self) -> u64 {
        self.db.last().ok()
            .flatten()
            .and_then(|(k, _)| {
                let ks = std::str::from_utf8(&k).ok()?;
                ks.strip_prefix("block:")?.parse().ok()
            })
            .unwrap_or(0)
    }

    pub fn load_chain(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        let mut h = 0u64;
        while let Some(b) = self.load_block(h) {
            blocks.push(b);
            h += 1;
        }
        blocks
    }
}
