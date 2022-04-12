use crate::entities::masternode::local_masternode::LocalMasternode;
use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct UpdateServiceTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub ip_address: &'a [u8],
    pub payload_signature: &'a [u8],
    pub port: i16,
    pub provider_registration_transaction_hash: &'a [u8],
    pub script_payout: &'a [u8],

    pub local_masternode: LocalMasternode<'a>,
}
