use dash_spv_primitives::crypto::{UInt256, UInt384, UInt768};
use crate::schema::quorums;

/// "quorumHashData == %@ && llmqType == %@ "
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Quorum {
    pub id: i32,
    pub block_id: i32,
    pub chain_id: i32,
    pub verified: bool,
    pub version: i16,
    pub all_commitment_aggregated_signature: UInt768,
    pub commitment_hash: UInt256,
    pub commitment_transaction_id: Option<i32>,
    pub quorum_index: Option<i32>,
    pub quorum_type: i16,
    pub quorum_hash: UInt256,
    pub quorum_public_key: UInt384,
    pub quorum_threshold_signature: UInt768,
    pub quorum_verification_vector_hash: UInt256,
    pub signers_count: i32,
    pub signers_bitset: Vec<u8>,
    pub valid_members_count: i32,
    pub valid_members_bitset: Vec<u8>,
    // pub instant_send_locks: HashSet<i32>,
    // pub chain_locks: HashSet<i32>,
    // pub referenced_by_masternode_lists: HashSet<i32>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="quorums"]
pub struct NewQuorum {
    pub block_id: i32,
    pub chain_id: i32,
    pub verified: bool,
    pub version: i16,
    pub all_commitment_aggregated_signature: UInt768,
    pub commitment_hash: UInt256,
    pub commitment_transaction_id: Option<i32>,
    pub quorum_index: Option<i32>,
    pub quorum_type: i16,
    pub quorum_hash: UInt256,
    pub quorum_public_key: UInt384,
    pub quorum_threshold_signature: UInt768,
    pub quorum_verification_vector_hash: UInt256,
    pub signers_count: i32,
    pub signers_bitset: Vec<u8>,
    pub valid_members_count: i32,
    pub valid_members_bitset: Vec<u8>,
}
