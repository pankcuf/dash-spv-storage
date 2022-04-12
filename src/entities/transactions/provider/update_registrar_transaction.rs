use crate::entities::masternode::local_masternode::LocalMasternode;
use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct UpdateRegistrarTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub operator_key: &'a [u8],
    pub payload_signature: &'a [u8],
    pub provider_mode: i16,
    pub provider_registration_transaction_hash: &'a [u8],
    pub script_payout: &'a [u8],
    pub voting_key_hash: &'a [u8],

    pub local_masternode: LocalMasternode<'a>,
}
