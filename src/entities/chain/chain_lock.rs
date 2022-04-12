use crate::entities::chain::chain::Chain;
use crate::entities::masternode::quorum_entry::QuorumEntry;
use crate::entities::merkle_block::MerkleBlock;

#[derive(Debug)]
pub struct ChainLock<'a> {
    pub signature: &'a [u8],
    pub valid_signature: bool,

    pub chain_if_last_chain_lock: Chain<'a>,
    pub merkle_block: MerkleBlock<'a>,
    pub quorum: QuorumEntry<'a>,

}
