use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use crate::schema::chains;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Chain {
    pub id: i32,
    pub chain_type: i16,
    pub version: i16,
    pub identifier: Option<String>,

    pub total_governance_objects_count: i32,
    pub last_chain_lock_id: Option<i32>,
    pub base_block_hash: Option<UInt256>,

    pub sync_block_chain_work: Option<UInt256>,
    pub sync_block_hash: Option<UInt256>,
    pub sync_block_height: Option<i32>,
    pub sync_block_timestamp: Option<NaiveDateTime>,
    pub sync_locators: Option<Vec<u8>>, //Vec<UInt256>

    // pub checkpoints: &'a [u8],
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="chains"]
pub struct NewChain<'a> {
    pub chain_type: i16,
    pub version: i16,
    pub identifier: Option<&'a str>,

    pub total_governance_objects_count: i32,
    pub last_chain_lock_id: Option<i32>,
    pub base_block_hash: Option<UInt256>,

    pub sync_block_chain_work: UInt256,
    pub sync_block_hash: UInt256,
    pub sync_block_height: i32,
    pub sync_block_timestamp: NaiveDateTime,
    pub sync_locators: Option<Vec<u8>>, //Vec<UInt256>

}
