use crate::entities::chain::chain::Chain;
use crate::entities::chain::chain_lock::ChainLock;
use crate::entities::masternode::masternode_list::MasternodeList;
use crate::entities::masternode::quorum_entry::QuorumEntry;

#[derive(Debug)]
pub struct MerkleBlock<'a> {
    pub block_hash: &'a [u8],
    pub chain_work: &'a [u8],
    pub flags: &'a [u8],
    pub hashes: &'a [u8],
    pub height: i32,
    pub merkle_root: &'a [u8],
    pub nonce: i32,
    pub prev_block: &'a [u8],
    pub target: i32,
    pub timestamp: i64,
    pub total_transactions: i32,
    pub version: i32,

    pub chain: Chain<'a>,
    pub chain_lock: ChainLock<'a>,
    pub masternode_list: MasternodeList<'a>,
    pub used_by_quorums: Vec<QuorumEntry<'a>>,
}
