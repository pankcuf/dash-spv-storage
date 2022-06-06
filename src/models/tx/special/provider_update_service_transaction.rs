use dash_spv_primitives::crypto::{UInt128, UInt256, UInt768};
use crate::schema::provider_update_service_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct ProviderUpdateServiceTransaction {
    pub id: i32,
    pub base_id: i32,

    pub local_masternode_id: i32,

    pub ip_address: UInt128,
    pub port: i16,

    pub provider_registration_transaction_hash: UInt256,
    pub payload_signature: UInt768,
    pub script_payout: Vec<u8>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="provider_update_service_transactions"]
pub struct NewProviderUpdateServiceTransaction {
    pub base_id: i32,

    pub local_masternode_id: i32,

    pub ip_address: UInt128,
    pub port: i16,

    pub provider_registration_transaction_hash: UInt256,
    pub payload_signature: UInt768,
    pub script_payout: Vec<u8>,
}
