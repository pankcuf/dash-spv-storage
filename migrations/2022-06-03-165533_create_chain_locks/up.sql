-- Your SQL goes here
CREATE TABLE chain_locks
(
    id INTEGER PRIMARY KEY NOT NULL,
    verified BOOLEAN NOT NULL,
    signature BLOB NOT NULL,
    block INTEGER NOT NULL,
    quorum_id INTEGER
)
