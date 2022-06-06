use dash_spv_primitives::crypto::UInt256;
use crate::schema::masternode_lists;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct MasternodeList {
    pub id: i32,
    pub block_id: i32,
    pub chain_id: i32,
    pub masternodes_merkle_root: UInt256,
    pub quorums_merkle_root: UInt256,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="masternode_lists"]
pub struct NewMasternodeList {
    pub block_id: i32,
    pub chain_id: i32,
    pub masternodes_merkle_root: UInt256,
    pub quorums_merkle_root: UInt256,
}
