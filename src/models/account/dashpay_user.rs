use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use crate::schema::dashpay_users;

/// "chain == %@"
/// "associatedBlockchainIdentity.uniqueID == %@"
/// "associatedBlockchainIdentity.uniqueID IN %@"
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct DashpayUser {
    pub id: i32,
    pub chain_id: i32,
    pub blockchain_identity_id: i32,

    pub local_profile_document_revision: i32,
    pub remote_profile_document_revision: i32,

    pub created_at: NaiveDateTime,//u64
    pub updated_at: NaiveDateTime,//u64

    //"maxLength": 2048
    pub avatar_path: Option<String>,
    //"maxLength": 25
    pub display_name: Option<String>,
    //"maxLength": 140
    pub public_message: Option<String>,

    pub avatar_fingerprint: Option<i64>,
    pub avatar_hash: Option<UInt256>,
    pub document_id: Option<UInt256>,
    pub original_entropy_data: Option<UInt256>,

}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="dashpay_users"]
pub struct NewDashpayUser<'a> {
    pub chain_id: i32,
    pub blockchain_identity_id: i32,
    pub local_profile_document_revision: i32,
    pub remote_profile_document_revision: i32,
    pub created_at: NaiveDateTime,//u64
    pub updated_at: NaiveDateTime,//u64
    //"maxLength": 2048
    pub avatar_path: Option<&'a str>,
    //"maxLength": 25
    pub display_name: Option<&'a str>,
    //"maxLength": 140
    pub public_message: Option<&'a str>,
    pub avatar_fingerprint: Option<i64>,
    pub avatar_hash: Option<UInt256>,
    pub document_id: Option<UInt256>,
    pub original_entropy_data: Option<UInt256>,
}
