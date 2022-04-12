use crate::entities::masternode::quorum_entry::QuorumEntry;
use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct QuorumCommitmentTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub quorum_commitment_height: i32,

    pub entry: QuorumEntry<'a>,
}
