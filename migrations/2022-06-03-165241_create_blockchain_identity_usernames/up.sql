-- Your SQL goes here
CREATE TABLE blockchain_identity_usernames (
    id INTEGER PRIMARY KEY NOT NULL,
    blockchain_identity_id INTEGER NOT NULL,
    blockchain_identity_id_used_for_dashpay INTEGER NOT NULL,
    status SMALLINT NOT NULL,
    domain VARCHAR NOT NULL,
    string_value VARCHAR NOT NULL,
    salt BLOB NOT NULL
)
