use dash_spv_primitives::crypto::{UInt160, UInt256, UInt384};
use crate::schema::provider_update_registrar_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct ProviderUpdateRegistrarTransaction {
    pub id: i32,
    pub base_id: i32,

    pub local_masternode_id: i32,

    pub provider_mode: i16,
    pub operator_key: UInt384,
    pub provider_registration_transaction_hash: UInt256,
    pub voting_key_hash: UInt160,

    pub payload_signature: Vec<u8>,
    pub script_payout: Vec<u8>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="provider_update_registrar_transactions"]
pub struct NewProviderUpdateRegistrarTransaction {
    pub base_id: i32,

    pub local_masternode_id: i32,

    pub provider_mode: i16,
    pub operator_key: UInt384,
    pub provider_registration_transaction_hash: UInt256,
    pub voting_key_hash: UInt160,

    pub payload_signature: Vec<u8>,
    pub script_payout: Vec<u8>,
}
