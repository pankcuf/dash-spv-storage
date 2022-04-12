use crate::entities::governance::vote_hash::VoteHash;
use crate::entities::masternode::masternode_entry::MasternodeEntry;

pub struct Vote<'a> {
    pub masternode_hash: &'a [u8],
    pub masternode_index: i32,
    pub outcome: i32,
    pub parent_hash: &'a [u8],
    pub signal: i32,
    pub signature: &'a [u8],
    pub timestamp_created: i64,

    pub governance_vote_hash: VoteHash<'a>,
    pub masternode: MasternodeEntry<'a>,
}
