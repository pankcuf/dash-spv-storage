use crate::entities::masternode::local_masternode::LocalMasternode;
use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct UpdateRevocationTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub payload_signature: &'a [u8],
    pub provider_registration_transaction_hash: &'a [u8],
    pub reason: i16,

    pub local_masternode: LocalMasternode<'a>,
}
