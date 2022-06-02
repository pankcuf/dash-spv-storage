use chrono::NaiveDateTime;
use super::schema::*;
use dash_spv_primitives::crypto::{BDictionary, Boolean, UInt128, UInt160, UInt256, UInt384, UInt768};

#[derive(Identifiable, Queryable, Associations, PartialEq, Eq, Debug)]
// #[table_name="masternodes"]
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

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
// #[belongs_to(Masternode)]
pub struct LocalMasternode {
    pub id: i32,
    pub operator_keys_index: i32,
    pub operator_keys_wallet_id: String,
    pub owner_keys_index: i32,
    pub owner_keys_wallet_id: String,
    pub holding_keys_index: i32,
    pub voting_keys_wallet_id: String,
    pub holding_keys_wallet_id: String,
    pub voting_keys_index: i32,
    pub provider_registration_transaction: i32,
    pub masternode_id: i32,
    // pub provider_update_registrar_transactions: BTreeSet<i32>,
    // pub provider_update_service_transactions: BTreeSet<i32>,
    // pub provider_update_revocation_transactions: BTreeSet<i32>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="local_masternodes"]
pub struct NewLocalMasternode<'a> {
    pub operator_keys_index: i32,
    pub operator_keys_wallet_id: &'a str,
    pub owner_keys_index: i32,
    pub owner_keys_wallet_id: &'a str,
    pub holding_keys_index: i32,
    pub voting_keys_wallet_id: &'a str,
    pub holding_keys_wallet_id: &'a str,
    pub voting_keys_index: i32,
    pub provider_registration_transaction: i32,
    pub masternode_id: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Quorum {
    pub id: i32,
    pub block_id: i32,
    pub chain_id: i32,
    pub verified: bool,
    pub version: i16,
    pub all_commitment_aggregated_signature: UInt768,
    pub commitment_hash: UInt256,
    pub commitment_transaction_id: Option<i32>,
    pub quorum_index: Option<i32>,
    pub quorum_type: i16,
    pub quorum_hash: UInt256,
    pub quorum_public_key: UInt384,
    pub quorum_threshold_signature: UInt768,
    pub quorum_verification_vector_hash: UInt256,
    pub signers_count: i32,
    pub signers_bitset: Vec<u8>,
    pub valid_members_count: i32,
    pub valid_members_bitset: Vec<u8>,
    // pub instant_send_locks: HashSet<i32>,
    // pub chain_locks: HashSet<i32>,
    // pub referenced_by_masternode_lists: HashSet<i32>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="quorums"]
pub struct NewQuorum {
    pub block_id: i32,
    pub chain_id: i32,
    pub verified: bool,
    pub version: i16,
    pub all_commitment_aggregated_signature: UInt768,
    pub commitment_hash: UInt256,
    pub commitment_transaction_id: Option<i32>,
    pub quorum_index: Option<i32>,
    pub quorum_type: i16,
    pub quorum_hash: UInt256,
    pub quorum_public_key: UInt384,
    pub quorum_threshold_signature: UInt768,
    pub quorum_verification_vector_hash: UInt256,
    pub signers_count: i32,
    pub signers_bitset: Vec<u8>,
    pub valid_members_count: i32,
    pub valid_members_bitset: Vec<u8>,
}

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct MasternodeList {
    pub id: i32,
    pub block_id: i32,
    pub chain_id: i32,
    pub masternodes_merkle_root: UInt256,
    pub quorums_merkle_root: UInt256,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="masternode_lists"]
pub struct NewMasternodeList {
    pub block_id: i32,
    pub chain_id: i32,
    pub masternodes_merkle_root: UInt256,
    pub quorums_merkle_root: UInt256,
}
