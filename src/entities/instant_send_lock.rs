use crate::entities::masternode::quorum_entry::QuorumEntry;
use crate::entities::transactions::transaction::Transaction;

#[derive(Debug)]
pub struct InstantSendLock<'a> {
    pub signature: &'a [u8],
    pub valid_signature: bool,

    pub quorum: QuorumEntry<'a>,
    pub transaction: Transaction<'a>,
}
