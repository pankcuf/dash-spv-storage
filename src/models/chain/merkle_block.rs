use std::ops::DerefMut;
use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};
use crate::get_pooled_connection;
use crate::schema::merkle_blocks;

/// queries:
/// "chain == %@"
/// "chain == %@ && (height == %@)"
/// "(chain == %@) && (blockHash == %@)"
/// "chain == %@ && (blockHash == %@ || blockHash == %@ )"
/// "(chain == %@) && (height > %u) && !(blockHash in %@)"
/// "(chain == %@) && masternodeList == NIL && (usedByQuorums.@count == 0) && !(blockHash in %@)"
/// "blockHash == %@"
/// "blockHash in %@"
/// indexation:
/// "height" DESC
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct MerkleBlock {
    pub id: i32,
    pub chain_id: i32,
    pub chain_lock_id: Option<i32>,
    pub masternode_list_id: Option<i32>,
    pub height: i32,
    pub block_hash: UInt256,
    pub chain_work: UInt256,
    pub merkle_root: UInt256,
    pub prev_block: UInt256,
    pub nonce: i32,
    pub target: i32,
    pub total_transactions: i32,
    pub version: i32,
    pub timestamp: NaiveDateTime,

    pub flags: Option<Vec<u8>>,
    pub hashes: Option<Vec<u8>>,

}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="merkle_blocks"]
pub struct NewMerkleBlock {
    pub chain_id: i32,
    pub chain_lock_id: Option<i32>,
    pub masternode_list_id: Option<i32>,
    pub height: i32,
    pub block_hash: UInt256,
    pub chain_work: UInt256,
    pub merkle_root: UInt256,
    pub prev_block: UInt256,
    pub nonce: i32,
    pub target: i32,
    pub total_transactions: i32,
    pub version: i32,
    pub timestamp: NaiveDateTime,

    pub flags: Option<Vec<u8>>,
    pub hashes: Option<Vec<u8>>,
}

impl MerkleBlock {
    pub fn merkle_block_with_hash(block_hash: UInt256) -> QueryResult<MerkleBlock> {
        let mut pooled_conn = get_pooled_connection();
        let predicate = merkle_blocks::block_hash.eq(block_hash);
        let selection = merkle_blocks::dsl::merkle_blocks::all_columns();
        merkle_blocks::dsl::merkle_blocks
            .select(selection)
            .filter(predicate)
            .first(pooled_conn.deref_mut())
    }
}
