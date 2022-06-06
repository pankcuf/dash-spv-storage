use dash_spv_primitives::crypto::UInt256;
use crate::schema::blockchain_identity_usernames;

/// "blockchainIdentity.uniqueID == %@"

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BlockchainIdentityUsername {
    pub id: i32,
    pub blockchain_identity_id: i32,
    pub blockchain_identity_id_used_for_dashpay: i32,
    pub status: i16,
    pub domain: String,
    pub string_value: String,
    pub salt: UInt256
}
#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="blockchain_identity_usernames"]
pub struct NewBlockchainIdentityUsername<'a> {
    pub blockchain_identity_id: i32,
    pub blockchain_identity_id_used_for_dashpay: i32,
    pub status: i16,
    pub domain: &'a str,
    pub string_value: &'a str,
    pub salt: UInt256
}
