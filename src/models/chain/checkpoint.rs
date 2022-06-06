use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use crate::schema::checkpoints;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Checkpoint {
    pub id: i32,
    pub chain_id: i32,
    pub height: i32,
    pub hash: UInt256,
    pub timestamp: NaiveDateTime,
    pub target: i32,
    pub masternode_list_path: Option<String>,
    pub merkle_root: Option<UInt256>,
    pub chain_work: UInt256,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="checkpoints"]
pub struct NewCheckpoint<'a> {
    pub chain_id: i32,
    pub height: i32,
    pub hash: UInt256,
    pub timestamp: NaiveDateTime,
    pub target: i32,
    pub masternode_list_path: Option<&'a str>,
    pub merkle_root: Option<UInt256>,
    pub chain_work: UInt256,
}
