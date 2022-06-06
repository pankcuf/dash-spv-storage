use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use crate::schema::governance_objects;
/// queries:
/// "governanceObjectHash.chain = %@"
/// "governanceObjectHash.governanceObjectHash = %@"
/// "governanceObjectHash.chain == %@ && type == %@"
/// indexation:
/// "timestamp" DESC
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct GovernanceObject {
    pub id: i32,
    pub chain_id: i32,
    pub amount: i64,
    pub start_epoch: i64,
    pub end_epoch: i64,
    pub revision: i32,
    pub timestamp: NaiveDateTime,
    pub total_votes_count: i64,

    pub object_type: i32,

    pub identifier: String,
    pub payment_address: String,
    pub url: String,

    pub collateral_hash: UInt256,
    pub parent_hash: UInt256,
    pub signature: Vec<u8>,

    /// migrated from GovernanceObjectHash
    pub hash: UInt256,
    pub hash_timestamp: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="governance_objects"]
pub struct NewGovernanceObject<'a> {
    pub chain_id: i32,
    pub amount: i64,
    pub start_epoch: i64,
    pub end_epoch: i64,
    pub revision: i32,
    pub timestamp: NaiveDateTime,
    pub total_votes_count: i64,

    pub object_type: i32,

    pub identifier: &'a str,
    pub payment_address: &'a str,
    pub url: &'a str,

    pub collateral_hash: UInt256,
    pub parent_hash: UInt256,
    pub signature: Vec<u8>,

    /// migrated from GovernanceObjectHash
    pub hash: UInt256,
    pub hash_timestamp: NaiveDateTime,
}
