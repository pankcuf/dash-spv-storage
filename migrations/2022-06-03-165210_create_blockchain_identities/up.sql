-- Your SQL goes here
CREATE TABLE blockchain_identities (
    id INTEGER PRIMARY KEY NOT NULL,
    chain_id INTEGER NOT NULL,
    is_local BOOLEAN NOT NULL,
    registration_status SMALLINT NOT NULL,
    credit_balance UNSIGNED BIG INT NOT NULL,
    dashpay_sync_block_hash BLOB NOT NULL,
    unique_id BLOB NOT NULL,
    last_checked_incoming_contacts_timestamp TIMESTAMP NOT NULL,
    last_checked_outgoing_contacts_timestamp TIMESTAMP NOT NULL,
    last_checked_profile_timestamp TIMESTAMP NOT NULL,
    last_checked_usernames_timestamp TIMESTAMP NOT NULL
)
