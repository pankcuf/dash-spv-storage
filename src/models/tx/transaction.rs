use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use crate::schema::transactions;
/// queries:
/// "transactionHash.txHash == %@"
/// "transactionHash.chain = %@"
/// "(ANY outputs.address == %@) || (ANY inputs.localAddress.address = %@)"
/// indexation:
/// "transactionHash.timestamp": DESC
/// ["transactionHash.blockHeight": DESC, "transactionHash.timestamp": DESC]
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Transaction {
    pub id: i32,

    pub lock_time: i32,
    pub associated_shapeshift_id: Option<i32>,
    pub instant_send_lock_id: Option<i32>,

    // from TransactionHash
    pub chain_id: i32,
    pub block_height: i32,
    pub timestamp: NaiveDateTime,
    pub hash: UInt256,

    pub version: i16, // special_transaction_version

    // special
    // pub special_transaction_version: i32,
    // pub height: i32,
    // pub registered_blockchain_identity: i32,
    // pub topped_up_blockchain_identity: i32,
    // pub operator_reward: i32,
    // pub port: i32,
    // pub port_1: i32,
    // pub provider_mode: i32,
    // pub provider_mode_1: i32,
    // pub provider_type: i32,
    // pub reason: i32,
    // pub local_masternode: i32,
    // pub local_masternode_1: i32,
    // pub local_masternode_2: i32,
    // pub fok_local_masternode: i32,
    // pub fok_local_masternode_1: i32,
    // pub fok_local_masternode_2: i32,
    // pub quorum_commitment_height: i32,
    // pub entry: i32,
    //
    // pub mn_list_merkle_root: UInt256,
    // pub collateral_outpoint: UInt256,
    // pub ip_address: UInt128,
    // pub ip_address_1: UInt128,
    // pub operator_key: UInt256,
    // pub operator_key_1: UInt256,
    // pub owner_key_hash: UInt256,
    //
    // pub voting_key_hash: UInt256,
    // pub voting_key_hash_1: UInt256,
    //
    // pub payload_signature: UInt256,
    // pub payload_signature_1: UInt256,
    // pub payload_signature_2: UInt256,
    // pub payload_signature_3: UInt256,
    //
    // pub script_payout: UInt256,
    // pub script_payout_1: UInt256,
    // pub script_payout_2: UInt256,
    //
    // pub provider_registration_transaction_hash: UInt256,
    // pub provider_registration_transaction_hash_1: UInt256,
    // pub provider_registration_transaction_hash_2: UInt256,



}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="transactions"]
pub struct NewTransaction {
    pub lock_time: i32,
    pub associated_shapeshift_id: Option<i32>,
    pub instant_send_lock_id: Option<i32>,

    // from TransactionHash
    pub chain_id: i32,
    pub block_height: i32,
    pub timestamp: NaiveDateTime,
    pub hash: UInt256,

    pub version: i16, // special_transaction_version
}
