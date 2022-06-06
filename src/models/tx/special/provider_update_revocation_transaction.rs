use dash_spv_primitives::crypto::{UInt256, UInt768};
use crate::schema::provider_update_revocation_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct ProviderUpdateRevocationTransaction {
    pub id: i32,
    pub base_id: i32,
    pub local_masternode_id: i32,

    pub reason: i16,
    pub provider_registration_transaction_hash: UInt256,
    pub payload_signature: UInt768,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="provider_update_revocation_transactions"]
pub struct NewProviderUpdateRevocationTransaction {
    pub base_id: i32,
    pub local_masternode_id: i32,

    pub reason: i16,
    pub provider_registration_transaction_hash: UInt256,
    pub payload_signature: UInt768,
}
