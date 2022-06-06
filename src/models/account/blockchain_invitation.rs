use crate::schema::blockchain_invitations;
/// "chain == %@"
/// "blockchainIdentity.uniqueID == %@"

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BlockchainInvitation {
    pub id: i32,
    pub chain_id: i32,
    pub blockchain_identity_id: i32,
    pub link: String,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="blockchain_invitations"]
pub struct NewBlockchainInvitation<'a> {
    pub chain_id: i32,
    pub blockchain_identity_id: i32,
    pub link: &'a str,
}
