-- DevilChain PostgreSQL Init
CREATE TABLE IF NOT EXISTS wallets (
    address VARCHAR(128) PRIMARY KEY,
    balance NUMERIC(36,18) DEFAULT 0,
    staked  NUMERIC(36,18) DEFAULT 0,
    tx_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS transactions (
    tx_hash VARCHAR(128) PRIMARY KEY,
    from_addr VARCHAR(128) NOT NULL,
    to_addr VARCHAR(128) NOT NULL,
    amount NUMERIC(36,18) NOT NULL,
    gas_fee NUMERIC(36,18) NOT NULL,
    block_height BIGINT,
    timestamp BIGINT,
    status VARCHAR(32) DEFAULT 'confirmed',
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS validators (
    address VARCHAR(128) PRIMARY KEY,
    staked_dvc NUMERIC(36,18) DEFAULT 0,
    reputation_score NUMERIC(10,4) DEFAULT 0,
    blocks_validated BIGINT DEFAULT 0,
    active BOOLEAN DEFAULT true,
    registered_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blocks (
    height BIGINT PRIMARY KEY,
    block_hash VARCHAR(128) UNIQUE NOT NULL,
    previous_hash VARCHAR(128),
    validator VARCHAR(128),
    tx_count INTEGER DEFAULT 0,
    nonce BIGINT,
    ai_score NUMERIC(5,4),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ai_scan_logs (
    id SERIAL PRIMARY KEY,
    target_type VARCHAR(32),
    target_hash VARCHAR(128),
    ai_score NUMERIC(5,4),
    verdict VARCHAR(32),
    warnings JSONB,
    scanned_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS fees (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(128),
    gas_fee NUMERIC(36,18),
    block_height BIGINT,
    timestamp BIGINT
);

-- Seed genesis validator
INSERT INTO validators (address, staked_dvc, active)
VALUES ('db1xval_genesis_001', 10000.0, true)
ON CONFLICT DO NOTHING;

-- Seed test wallets
INSERT INTO wallets (address, balance) VALUES
  ('db1xtest_alice_001', 5000.0),
  ('db1xtest_bob_002', 2500.0),
  ('db1xtest_miner_001', 1000.0),
  ('db1xdao_treasury', 150000000.0)
ON CONFLICT DO NOTHING;
