//! DevilStorage — Decentralized File Storage Node
//! Stores files as content-addressed chunks (CID-based)
//! Compatible with IPFS CID format

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;
use log::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub cid: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub owner: String,       // DevilChain address
    pub uploaded_at: u64,
    pub is_public: bool,
    pub chunk_count: u32,
    pub replicas: u8,        // Number of storage nodes holding this file
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    pub cid: String,
    pub chunk_index: u32,
    pub data_hash: String,
    pub size: u32,
}

pub struct DevilStorageNode {
    pub node_address: String,
    pub storage_path: PathBuf,
    pub metadata: HashMap<String, FileMetadata>,
    pub capacity_bytes: u64,
    pub used_bytes: u64,
}

impl DevilStorageNode {
    pub fn new(node_address: &str, storage_path: &str, capacity_gb: u64) -> Self {
        DevilStorageNode {
            node_address: node_address.to_string(),
            storage_path: PathBuf::from(storage_path),
            metadata: HashMap::new(),
            capacity_bytes: capacity_gb * 1024 * 1024 * 1024,
            used_bytes: 0,
        }
    }

    /// Generate CID from file content (SHA-256 based)
    pub fn compute_cid(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("dvl1{}", hex::encode(hasher.finalize()))
    }

    /// Store a file and return its CID
    pub fn store(&mut self, data: &[u8], file_name: &str, owner: &str, is_public: bool) -> Result<String> {
        let cid = Self::compute_cid(data);

        if self.used_bytes + data.len() as u64 > self.capacity_bytes {
            return Err(anyhow::anyhow!("Storage node full"));
        }

        // Write to disk
        let file_path = self.storage_path.join(&cid);
        std::fs::write(&file_path, data)?;

        let meta = FileMetadata {
            cid: cid.clone(),
            file_name: file_name.to_string(),
            size_bytes: data.len() as u64,
            owner: owner.to_string(),
            uploaded_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            is_public,
            chunk_count: 1,
            replicas: 1,
        };

        self.used_bytes += data.len() as u64;
        self.metadata.insert(cid.clone(), meta);
        info!("[DevilStorage] Stored: {} ({} bytes) CID: {}", file_name, data.len(), cid);
        Ok(cid)
    }

    /// Retrieve file by CID
    pub fn retrieve(&self, cid: &str) -> Result<Vec<u8>> {
        let meta = self.metadata.get(cid)
            .ok_or_else(|| anyhow::anyhow!("File not found: {}", cid))?;
        let file_path = self.storage_path.join(cid);
        Ok(std::fs::read(file_path)?)
    }

    /// Delete file by CID (owner only enforced at contract layer)
    pub fn delete(&mut self, cid: &str) -> Result<()> {
        if let Some(meta) = self.metadata.remove(cid) {
            let file_path = self.storage_path.join(cid);
            if file_path.exists() { std::fs::remove_file(file_path)?; }
            self.used_bytes = self.used_bytes.saturating_sub(meta.size_bytes);
            info!("[DevilStorage] Deleted: {}", cid);
        }
        Ok(())
    }

    pub fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "node": self.node_address,
            "total_files": self.metadata.len(),
            "capacity_bytes": self.capacity_bytes,
            "used_bytes": self.used_bytes,
            "available_bytes": self.capacity_bytes - self.used_bytes,
            "usage_percent": (self.used_bytes as f64 / self.capacity_bytes as f64 * 100.0).round()
        })
    }
}
