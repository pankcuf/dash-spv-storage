use dash_spv_primitives::crypto::{UInt128, UInt160, UInt256, UInt384};
use crate::schema::provider_registration_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct ProviderRegistrationTransaction {
    pub id: i32,
    pub base_id: i32,

    pub local_masternode_id: i32,

    pub provider_mode: i16,
    pub provider_type: i16,
    pub ip_address: UInt128,
    pub port: i16,
    pub operator_reward: i16,
    pub collateral_outpoint: UInt256,
    pub operator_key: UInt384,
    pub owner_key_hash: UInt160,
    pub voting_key_hash: UInt160,

    pub payload_signature: Vec<u8>,
    pub script_payout: Vec<u8>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="provider_registration_transactions"]
pub struct NewProviderRegistrationTransaction {
    pub base_id: i32,

    pub local_masternode_id: i32,

    pub provider_mode: i16,
    pub provider_type: i16,
    pub ip_address: UInt128,
    pub port: i16,
    pub operator_reward: i16,
    pub collateral_outpoint: UInt256,
    pub operator_key: UInt384,
    pub owner_key_hash: UInt160,
    pub voting_key_hash: UInt160,

    pub payload_signature: Vec<u8>,
    pub script_payout: Vec<u8>,
}
