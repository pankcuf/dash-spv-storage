-- Your SQL goes here
CREATE TABLE local_masternodes (
    id INTEGER PRIMARY KEY NOT NULL,
    holding_keys_index INTEGER,
    holding_keys_wallet_id VARCHAR,
    operator_keys_index INTEGER,
    operator_keys_wallet_id VARCHAR,
    owner_keys_index INTEGER,
    owner_keys_wallet_id VARCHAR,
    voting_keys_index INTEGER,
    voting_keys_wallet_id VARCHAR,
    provider_registration_transaction INTEGER,
    masternode_id INTEGER
)
