use crate::entities::masternode::masternode_entry::MasternodeEntry;
use crate::entities::transactions::provider::registration_transaction::RegistrationTransaction;
use crate::entities::transactions::provider::update_registrar_transaction::UpdateRegistrarTransaction;
use crate::entities::transactions::provider::update_revocation_transaction::UpdateRevocationTransaction;
use crate::entities::transactions::provider::update_service_transaction::UpdateServiceTransaction;

#[derive(Debug)]
pub struct LocalMasternode<'a> {
    pub holding_keys_index: i32,
    pub holding_keys_wallet_unique_id: &'a str,
    pub operator_keys_index: i32,
    pub operator_keys_wallet_unique_id: &'a str,
    pub owner_keys_index: i32,
    pub owner_keys_wallet_unique_id: &'a str,
    pub voting_keys_index: i32,
    pub voting_keys_wallet_unique_id: &'a str,

    pub provider_registration_transaction: RegistrationTransaction<'a>,
    pub provider_update_registrar_transactions: Vec<UpdateRegistrarTransaction<'a>>,
    pub provider_update_revocation_transactions: Vec<UpdateRevocationTransaction<'a>>,
    pub provider_update_service_transactions: Vec<UpdateServiceTransaction<'a>>,
    pub masternode_entry: MasternodeEntry<'a>,
}
