use crate::entities::masternode::local_masternode::LocalMasternode;
use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct RegistrationTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub collateral_outpoint: &'a [u8],
    pub ip_address: &'a [u8],
    pub operator_key: &'a [u8],
    pub operator_reward: i16,
    pub owner_key_hash: &'a [u8],
    pub payload_signature: &'a [u8],
    pub port: i16,
    pub provider_mode: i16,
    pub provider_type: i16,
    pub script_payout: &'a [u8],
    pub voting_key_hash: &'a [u8],

    pub local_masternode: LocalMasternode<'a>,
}
