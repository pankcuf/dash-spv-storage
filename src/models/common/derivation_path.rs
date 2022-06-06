use crate::schema::derivation_paths;

/// "(chain == %@)"
/// "publicKeyIdentifier == %@ && chain == %@"
///
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct DerivationPath {
    pub id: i32,
    pub chain_id: i32,
    pub account_id: Option<i32>,
    pub friend_request_id: Option<i32>,
    pub sync_block_height: i32,
    pub public_key_identifier: String,
    pub derivation_path: Vec<u8>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="derivation_paths"]
pub struct NewDerivationPath<'a> {
    pub chain_id: i32,
    pub sync_block_height: i32,
    pub account_id: Option<i32>,
    pub friend_request_id: Option<i32>,
    pub public_key_identifier: &'a str,
    pub derivation_path: Vec<u8>,
}
