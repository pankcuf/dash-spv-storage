-- Your SQL goes here
CREATE TABLE transactions
(
    id INTEGER PRIMARY KEY NOT NULL,
    lock_time INTEGER,
    associated_shapeshift_id INTEGER,
    instant_send_lock_id INTEGER,
    chain_id INTEGER NOT NULL,
    block_height INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    hash BLOB NOT NULL,
    version SMALLINT
)
