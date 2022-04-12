use crate::entities::masternode::masternode_entry::MasternodeEntry;
use crate::entities::masternode::quorum_entry::QuorumEntry;
use crate::entities::merkle_block::MerkleBlock;

#[derive(Debug)]
pub struct MasternodeList<'a> {
    pub masternode_list_merkle_root: &'a [u8],
    pub quorum_list_merkle_root: &'a [u8],

    pub block: MerkleBlock<'a>,
    pub masternodes: Vec<MasternodeEntry<'a>>,
    pub quorums: Vec<QuorumEntry<'a>>,
}
