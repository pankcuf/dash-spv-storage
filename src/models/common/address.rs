use dash_spv_primitives::crypto::Boolean;
use crate::schema::addresses;

/// queries
/// "(derivationPath == %@)"
/// "(derivationPath == %@) && (identityIndex == %@)"
/// "(derivationPath == %@) && (internal == %@)"
/// "derivationPath.chain == %@"
/// "address == %@ && derivationPath.chain == %@"
/// "address IN %@ && derivationPath.chain == %@"
/// indexation
/// "index": ASC

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
#[table_name="addresses"]
pub struct Address {
    pub id: i32,
    pub identity_index: i32,
    pub index: i32,
    pub derivation_path: i32,
    pub address: String,
    pub internal: Boolean,
    pub standalone: Boolean,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="addresses"]
pub struct NewAddress<'a> {
    pub identity_index: i32,
    pub index: i32,
    pub derivation_path: i32,
    pub address: &'a str,
    pub internal: Boolean,
    pub standalone: Boolean,
}
