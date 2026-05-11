//! DevilID — Decentralized Identity module (off-chain Rust layer)
//! Mirrors DevilID.sol with local verification and DID document generation

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use crate::util::now_timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDDocument {
    pub id: String,            // did:devil:db1x...
    pub controller: String,    // DevilChain address
    pub username: String,
    pub display_name: String,
    pub avatar_cid: String,
    pub bio: String,
    pub created_at: u64,
    pub reputation: u64,
    pub dao_verified: bool,
    pub credentials: Vec<VerifiableCredential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub id: String,
    pub issuer: String,
    pub subject: String,
    pub credential_type: String,
    pub data_hash: String,
    pub issued_at: u64,
    pub expires_at: u64,
    pub revoked: bool,
}

pub struct DevilIDRegistry {
    pub identities: HashMap<String, DIDDocument>,   // address -> DID
    pub username_index: HashMap<String, String>,    // username -> address
}

impl DevilIDRegistry {
    pub fn new() -> Self {
        DevilIDRegistry {
            identities: HashMap::new(),
            username_index: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        address: &str,
        username: &str,
        display_name: &str,
        bio: &str,
        avatar_cid: &str,
    ) -> Result<String, String> {
        if self.identities.contains_key(address) {
            return Err("Identity already exists".to_string());
        }
        if self.username_index.contains_key(username) {
            return Err("Username taken".to_string());
        }
        if username.len() < 3 {
            return Err("Username too short".to_string());
        }

        let did = format!("did:devil:{}", address);
        let doc = DIDDocument {
            id: did.clone(),
            controller: address.to_string(),
            username: username.to_string(),
            display_name: display_name.to_string(),
            avatar_cid: avatar_cid.to_string(),
            bio: bio.to_string(),
            created_at: now_timestamp(),
            reputation: 0,
            dao_verified: false,
            credentials: vec![],
        };

        self.identities.insert(address.to_string(), doc);
        self.username_index.insert(username.to_string(), address.to_string());
        Ok(did)
    }

    pub fn resolve(&self, address: &str) -> Option<&DIDDocument> {
        self.identities.get(address)
    }

    pub fn resolve_username(&self, username: &str) -> Option<&DIDDocument> {
        self.username_index.get(username)
            .and_then(|addr| self.identities.get(addr))
    }

    pub fn dao_verify(&mut self, address: &str) {
        if let Some(doc) = self.identities.get_mut(address) {
            doc.dao_verified = true;
            doc.reputation += 100;
        }
    }

    pub fn issue_credential(
        &mut self,
        issuer: &str,
        subject: &str,
        cred_type: &str,
        data: &str,
        expires_at: u64,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", issuer, subject, cred_type, data));
        let cred_id = hex::encode(hasher.finalize());

        let cred = VerifiableCredential {
            id: cred_id.clone(),
            issuer: issuer.to_string(),
            subject: subject.to_string(),
            credential_type: cred_type.to_string(),
            data_hash: {
                let mut h = Sha256::new(); h.update(data); hex::encode(h.finalize())
            },
            issued_at: now_timestamp(),
            expires_at,
            revoked: false,
        };

        if let Some(doc) = self.identities.get_mut(subject) {
            doc.credentials.push(cred);
        }
        cred_id
    }

    pub fn revoke_credential(&mut self, subject: &str, cred_id: &str) {
        if let Some(doc) = self.identities.get_mut(subject) {
            if let Some(c) = doc.credentials.iter_mut().find(|c| c.id == cred_id) {
                c.revoked = true;
            }
        }
    }
}

mod util {
    pub fn now_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
