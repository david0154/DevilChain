//! DevilChain Wallet — Real Ed25519 cryptography
//! Uses OsRng (cryptographically secure), real keypairs, BIP39-compatible mnemonic
//!
//! Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;          // ✅ cryptographically secure RNG
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

// ── Mnemonic word list (BIP39-compatible, first 64 words for demo) ────────────
const MNEMONIC_WORDS: &[&str] = &[
    "abandon","ability","able","about","above","absent","absorb","abstract",
    "absurd","abuse","access","accident","account","accuse","achieve","acid",
    "acoustic","acquire","across","act","action","actor","actress","actual",
    "adapt","add","addict","address","adjust","admit","adult","advance",
    "advice","aerobic","affair","afford","afraid","again","age","agent",
    "agree","ahead","aim","air","airport","aisle","alarm","album",
    "alcohol","alert","alien","all","alley","allow","almost","alone",
    "alpha","already","also","alter","always","amateur","amazing","among",
];

// ── Keypair ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletKeypair {
    pub address:     String,   // db1x{32 hex chars}
    pub public_key:  String,   // hex(Ed25519 32-byte verifying key)
    pub private_key: String,   // hex(Ed25519 32-byte signing key seed) — NEVER expose!
    pub mnemonic:    String,   // 12-word phrase
    pub balance:     u128,     // micro-DVC (populated from ledger)
    pub nonce:       u64,
}

impl WalletKeypair {
    /// Generate a brand-new cryptographically secure wallet
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);   // ✅ OsRng
        let verifying_key: VerifyingKey = signing_key.verifying_key();

        let priv_hex = hex::encode(signing_key.as_bytes());
        let pub_hex  = hex::encode(verifying_key.as_bytes());

        // Derive address: db1x + first 32 hex chars of SHA256(pubkey)
        let addr_hash = Sha256::digest(verifying_key.as_bytes());
        let address = format!("db1x{}", &hex::encode(addr_hash)[..32]);

        let mnemonic = Self::generate_mnemonic(signing_key.as_bytes());

        Self {
            address, public_key: pub_hex, private_key: priv_hex,
            mnemonic, balance: 0, nonce: 0,
        }
    }

    /// Restore wallet from private key hex
    pub fn from_private_key(priv_hex: &str) -> Result<Self, &'static str> {
        let bytes = hex::decode(priv_hex).map_err(|_| "Invalid hex")?;
        let arr: [u8; 32] = bytes.try_into().map_err(|_| "Key must be 32 bytes")?;
        let signing_key = SigningKey::from_bytes(&arr);
        let verifying_key = signing_key.verifying_key();
        let pub_hex  = hex::encode(verifying_key.as_bytes());
        let addr_hash = Sha256::digest(verifying_key.as_bytes());
        let address = format!("db1x{}", &hex::encode(addr_hash)[..32]);
        let mnemonic = Self::generate_mnemonic(&arr);
        Ok(Self {
            address, public_key: pub_hex, private_key: priv_hex.to_string(),
            mnemonic, balance: 0, nonce: 0,
        })
    }

    /// Sign a message hash — returns hex(signature)
    pub fn sign(&self, message: &str) -> Result<String, &'static str> {
        let bytes = hex::decode(&self.private_key).map_err(|_| "Bad key")?;
        let arr: [u8; 32] = bytes.try_into().map_err(|_| "Key len")?;
        let signing_key = SigningKey::from_bytes(&arr);
        let sig = signing_key.sign(message.as_bytes());
        Ok(hex::encode(sig.to_bytes()))
    }

    /// Derive 12-word mnemonic from private key bytes (deterministic)
    fn generate_mnemonic(seed: &[u8]) -> String {
        let n = MNEMONIC_WORDS.len();
        (0..12)
            .map(|i| {
                let idx = (seed[i % seed.len()] as usize
                    ^ (seed[(i + 1) % seed.len()] as usize) << 2) % n;
                MNEMONIC_WORDS[idx]
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// ── Miner Identity ────────────────────────────────────────────────────────────

/// Miner registration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerIdentity {
    pub address:       String,
    pub public_key:    String,
    pub name:          String,
    pub pool_address:  String,     // mining pool or solo
    pub stake:         u128,       // micro-DVC staked
    pub blocks_mined:  u64,
    pub total_earned:  u128,       // micro-DVC lifetime earnings
    pub registered_at: u64,
}

impl MinerIdentity {
    pub fn new(wallet: &WalletKeypair, name: &str) -> Self {
        Self {
            address:       wallet.address.clone(),
            public_key:    wallet.public_key.clone(),
            name:          name.to_string(),
            pool_address:  crate::tokenomics::MINING_POOL_WALLET.to_string(),
            stake:         0,
            blocks_mined:  0,
            total_earned:  0,
            registered_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }
}
