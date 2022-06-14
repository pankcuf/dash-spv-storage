use std::ops::DerefMut;
use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};
use diesel::dsl::count;
use crate::{create, delete, get_pooled_connection, schema};
use schema::*;
use crate::models::account::blockchain_identity_key_path::{count_key_paths_with_key_id, NewBlockchainIdentityKeyPath};

/// "chain == %@"
/// "uniqueID == %@"
/// "chain == %@ && isLocal == FALSE"
/// "chain == %@ && isLocal == TRUE"
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
#[table_name="blockchain_identities"]
pub struct BlockchainIdentity {
    pub id: i32,
    pub chain_id: i32,
    pub is_local: bool,
    pub registration_status: i16,
    pub credit_balance: i64,
    pub dashpay_sync_block_hash: UInt256,
    pub unique_id: UInt256,
    pub last_checked_incoming_contacts_timestamp: NaiveDateTime,
    pub last_checked_outgoing_contacts_timestamp: NaiveDateTime,
    pub last_checked_profile_timestamp: NaiveDateTime,
    pub last_checked_usernames_timestamp: NaiveDateTime,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="blockchain_identities"]
pub struct NewBlockchainIdentity {
    pub chain_id: i32,
    pub credit_balance: i64,
    pub dashpay_sync_block_hash: UInt256,
    pub is_local: bool,
    pub last_checked_incoming_contacts_timestamp: NaiveDateTime,
    pub last_checked_outgoing_contacts_timestamp: NaiveDateTime,
    pub last_checked_profile_timestamp: NaiveDateTime,
    pub last_checked_usernames_timestamp: NaiveDateTime,
    pub registration_status: i16,
    pub unique_id: UInt256,
}

pub fn delete_blockchain_identities_for_chain(chain_id: i32) -> QueryResult<usize> {
    let predicate = blockchain_identities::chain_id.eq(chain_id);
    let source = blockchain_identities::dsl::blockchain_identities.filter(predicate);
    delete(source)
}

pub fn blockchain_identity_with_unique_id(unique_id: UInt256) -> QueryResult<BlockchainIdentity> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = blockchain_identities::unique_id.eq(unique_id.clone());
    blockchain_identities::dsl::blockchain_identities.select(blockchain_identities::dsl::blockchain_identities::all_columns())
        .filter(predicate)
        .first::<BlockchainIdentity>(pooled_conn.deref_mut())
}

pub fn load_external_blockchain_identities(chain_id: i32) -> QueryResult<Vec<BlockchainIdentity>> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = blockchain_identities::chain_id.eq(chain_id)
        .and(blockchain_identities::is_local.eq(false));
    blockchain_identities::dsl::blockchain_identities.select(blockchain_identities::dsl::blockchain_identities::all_columns())
        .filter(predicate)
        .get_results::<BlockchainIdentity>(pooled_conn.deref_mut())

}

pub fn count_local_identities(chain_id: i32) -> QueryResult<i64> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = blockchain_identities::chain_id.eq(chain_id)
        .and(blockchain_identities::is_local.eq(true));
    blockchain_identities::dsl::blockchain_identities.select(count(predicate))
        .first(pooled_conn.deref_mut())
}

pub fn save_new_remote_identity_key(
    unique_id: UInt256,
    key_id: i32,
    key_status: i16,
    key_type: i16,
    public_key: Vec<u8>) -> QueryResult<usize> {
    if let Ok(identity) = blockchain_identity_with_unique_id(unique_id) {
        let blockchain_identity_id = identity.id;
        if let Ok(count) = count_key_paths_with_key_id(blockchain_identity_id, key_id) {
            if count == 0 {
                return create(blockchain_identity_key_paths::dsl::blockchain_identity_key_paths, &NewBlockchainIdentityKeyPath {
                    blockchain_identity_id,
                    derivation_path_id: None,
                    key_id,
                    key_status,
                    key_type,
                    public_key,
                    path: vec![]
                })
            }
        }
    }
    Ok(0)
}
