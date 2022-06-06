-- Your SQL goes here
CREATE TABLE credit_funding_transactions
(
    id INTEGER PRIMARY KEY NOT NULL,
    base_id INTEGER NOT NULL,
    registered_blockchain_identity_id INTEGER NOT NULL,
    topped_up_blockchain_identity_id INTEGER NOT NULL
)
