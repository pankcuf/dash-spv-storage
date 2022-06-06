use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::{BDictionary, Boolean, UInt128, UInt160, UInt256, UInt384};
use crate::schema::masternodes;

#[derive(Identifiable, Queryable, Associations, PartialEq, Eq, Debug)]
// #[belongs_to(LocalMasternode)]
pub struct Masternode<'a> {
    pub id: i32,
    pub chain_id: i32,
    pub address: i64,
    pub port: i16,
    pub core_last_connection_date: Option<NaiveDateTime>,
    pub core_protocol: i64,
    pub core_version: Option<String>,
    pub is_valid: bool,
    pub platform_ping: i64,
    pub platform_ping_date: Option<NaiveDateTime>,
    pub platform_version: Option<String>,
    pub known_confirmed_at_height: i32,
    pub update_height: i32,
    pub local_masternode_id: Option<i32>,
    pub prev_operator_bls_public_keys: BDictionary<'a, UInt256, UInt384>,
    pub prev_masternode_entry_hashes: BDictionary<'a, UInt256, UInt256>,
    pub prev_validity: BDictionary<'a, UInt256, Boolean>,
    pub confirmed_hash: UInt256,
    pub ipv6_address: UInt128,
    pub key_id_voting: UInt160,
    pub operator_bls_public_key: UInt384,
    pub provider_registration_transaction_hash: UInt256,
    pub masternode_entry_hash: UInt256,

    // pub addresses: BTreeSet<i32>,
    // pub governance_votes: HashSet<i32>,
    // pub masternode_lists: HashSet<i32>,
}

#[derive(Insertable, Associations, PartialEq, Eq, Debug)]
#[table_name="masternodes"]
pub struct NewMasternode<'a> {
    pub chain_id: i32,
    pub address: i64,
    pub port: i16,
    pub core_last_connection_date: Option<NaiveDateTime>,
    pub core_protocol: i64,
    pub core_version: Option<&'a str>,
    pub is_valid: bool,
    pub platform_ping: i64,
    pub platform_ping_date: Option<NaiveDateTime>,
    pub platform_version: Option<&'a str>,
    pub known_confirmed_at_height: i32,
    pub update_height: i32,
    pub local_masternode_id: Option<i32>,
    pub prev_operator_bls_public_keys: BDictionary<'a, UInt256, UInt384>,
    pub prev_masternode_entry_hashes: BDictionary<'a, UInt256, UInt256>,
    pub prev_validity: BDictionary<'a, UInt256, Boolean>,
    pub confirmed_hash: UInt256,
    pub ipv6_address: UInt128,
    pub key_id_voting: UInt160,
    pub operator_bls_public_key: UInt384,
    pub provider_registration_transaction_hash: UInt256,
    pub masternode_entry_hash: UInt256,
}
