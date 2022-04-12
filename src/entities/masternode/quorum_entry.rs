use crate::entities::chain::chain::Chain;
use crate::entities::chain::chain_lock::ChainLock;
use crate::entities::instant_send_lock::InstantSendLock;
use crate::entities::masternode::masternode_list::MasternodeList;
use crate::entities::merkle_block::MerkleBlock;
use crate::entities::transactions::quorum_commitment_transaction::QuorumCommitmentTransaction;

#[derive(Debug)]
pub struct QuorumEntry<'a> {
    pub all_commitment_aggregated_signature: &'a [u8],
    pub commitment_hash: &'a [u8],
    pub llmq_type: i16,
    pub quorum_hash: &'a [u8],
    pub quorum_public_key: &'a [u8],
    pub quorum_threshold_signature: &'a [u8],
    pub quorum_verification_vector_hash: &'a [u8],
    pub signers_bitset: &'a [u8],
    pub signers_count: i32,
    pub valid_members_bitset: &'a [u8],
    pub valid_members_count: i32,
    pub verified: bool,
    pub version: i16,

    pub block: MerkleBlock<'a>,
    pub chain: Chain<'a>,
    pub chain_locks: Vec<ChainLock<'a>>,
    pub commitment_transaction: QuorumCommitmentTransaction<'a>,
    pub instant_send_locks: Vec<InstantSendLock<'a>>,
    pub referenced_by_masternode_lists: Vec<MasternodeList<'a>>,
}
