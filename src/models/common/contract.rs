use dash_spv_primitives::crypto::UInt256;
use crate::schema::contracts;

/// "localContractIdentifier == %@ && chain == %@"
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Contract {
    pub id: i32,
    pub chain_id: i32,
    pub creator_id: i32, //blockchain_identity
    pub state: i16,
    pub local_contract_id: String,
    pub registered_blockchain_identity_unique_id: Option<UInt256>,
    pub entropy: Option<UInt256>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="contracts"]
pub struct NewContract<'a> {
    pub chain_id: i32,
    pub creator_id: i32,
    pub state: i16,
    pub local_contract_id: &'a str,
    pub registered_blockchain_identity_unique_id: Option<UInt256>,
    pub entropy: Option<UInt256>,
}
