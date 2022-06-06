use dash_spv_primitives::crypto::UInt768;
use crate::schema::chain_locks;

/// "merkleBlock.blockHash == %@"

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct ChainLock {
    pub id: i32,
    pub verified: bool,
    pub signature: UInt768,
    pub block: i32,
    pub quorum_id: Option<i32>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="chain_locks"]
pub struct NewChainLock {
    pub verified: bool,
    pub signature: UInt768,
    pub block: i32,
    pub quorum_id: Option<i32>,
}
