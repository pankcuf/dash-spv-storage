use std::ops::DerefMut;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryResult, RunQueryDsl};
use diesel::dsl::count;
use diesel::query_dsl::methods::SelectDsl;
use crate::get_pooled_connection;
use crate::schema::blockchain_identity_key_paths;
/// queries:
/// "blockchainIdentity == %@ && derivationPath == %@ && path == %@"
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct BlockchainIdentityKeyPath {
    pub id: i32,
    pub blockchain_identity_id: i32,
    pub derivation_path_id: Option<i32>,
    pub key_id: i32,
    pub key_status: i16,
    pub key_type: i16,
    pub public_key: Vec<u8>, //bls: u384 ecdsa: u256
    pub path: Vec<u8>,
}
#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="blockchain_identity_key_paths"]
pub struct NewBlockchainIdentityKeyPath {
    pub blockchain_identity_id: i32,
    pub derivation_path_id: Option<i32>,
    pub key_id: i32,
    pub key_status: i16,
    pub key_type: i16,
    pub public_key: Vec<u8>, //bls: u384 ecdsa: u256
    pub path: Vec<u8>,
}

pub fn count_key_paths_for(blockchain_identity_id: i32, derivation_path_id: i32, path: Vec<u8>) -> QueryResult<i64> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = blockchain_identity_key_paths::blockchain_identity_id.eq(blockchain_identity_id)
        .and(blockchain_identity_key_paths::derivation_path_id.eq(Some(derivation_path_id)))
        .and(blockchain_identity_key_paths::path.eq(path));
    blockchain_identity_key_paths::dsl::blockchain_identity_key_paths
        .select(count(predicate))
        .first(pooled_conn.deref_mut())
}

pub fn count_key_paths_with_key_id(blockchain_identity_id: i32, key_id: i32) -> QueryResult<i64> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = blockchain_identity_key_paths::blockchain_identity_id.eq(blockchain_identity_id)
        .and(blockchain_identity_key_paths::key_id.eq(key_id));
    blockchain_identity_key_paths::dsl::blockchain_identity_key_paths
        .select(count(predicate))
        .first(pooled_conn.deref_mut())
}
