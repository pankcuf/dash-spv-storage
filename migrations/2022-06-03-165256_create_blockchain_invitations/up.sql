-- Your SQL goes here
CREATE TABLE blockchain_invitations (
    id INTEGER PRIMARY KEY NOT NULL,
    blockchain_identity_id INTEGER NOT NULL,
    chain_id INTEGER NOT NULL,
    link VARCHAR NOT NULL
)
