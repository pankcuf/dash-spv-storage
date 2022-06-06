use std::ops::DerefMut;
use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::{BDictionary, Boolean, UInt128, UInt160, UInt256, UInt384, UInt768};
use diesel::{BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl, QueryResult, RunQueryDsl, Table};
use crate::connection_manager::get_pooled_connection;
use crate::models::masternode::{Masternode, MasternodeList, NewMasternode, NewMasternodeList, NewQuorum, Quorum};

mod schema;
mod test;
pub mod models;
pub mod connection_manager;

#[macro_use] extern crate diesel;


///
/// Common methods
///

pub fn delete<T>(source: T) -> QueryResult<usize>
    where
        T: diesel::query_builder::IntoUpdateTarget + diesel::associations::HasTable,
        T::Table: diesel::query_builder::QueryId + diesel::QuerySource,
        T::WhereClause: diesel::query_builder::QueryId + diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
        <T::Table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite> {
    let mut pooled_conn = get_pooled_connection();
    diesel::delete(source)
        .execute(pooled_conn.deref_mut())
}

pub fn create<T, U>(target: T, records: U) -> QueryResult<usize>
    where
        T: Table + diesel::QuerySource,
        T::FromClause: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
        U: Insertable<T>,
        U::Values: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite> + diesel::insertable::CanInsertInSingleQuery<diesel::sqlite::Sqlite> {
    let mut pooled_conn = get_pooled_connection();
    diesel::insert_into(target)
        .values(records)
        .execute(pooled_conn.deref_mut())
}

///
/// MasternodeList methods
///

pub fn create_masternode_list(block_id: i32,
                              chain_id: i32,
                              masternodes_merkle_root: UInt256,
                              quorums_merkle_root: UInt256) -> QueryResult<usize> {
    let target = schema::masternode_lists::dsl::masternode_lists;
    let records = NewMasternodeList { block_id, chain_id, masternodes_merkle_root, quorums_merkle_root };
    create(target, &records)
}

pub fn delete_masternode_list(chain_id: i32, block_id: i32) -> QueryResult<usize> {
    let predicate = schema::masternode_lists::chain_id.eq(chain_id)
        .and(schema::masternode_lists::block_id.eq(block_id));
    let source = schema::masternode_lists::dsl::masternode_lists.filter(predicate);
    delete(source)
}

pub fn delete_masterenode_lists_for_chain(chain_id: i32) -> QueryResult<usize> {
    let source = schema::masternode_lists::dsl::masternode_lists.filter(schema::masternode_lists::chain_id.eq(chain_id));
    delete(source)
}

pub fn update_masternode_list(block_id: i32, chain_id: i32, masternodes_root: UInt256, quorums_root: UInt256) -> QueryResult<usize> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = schema::masternode_lists::chain_id.eq(chain_id)
        .and(schema::masternode_lists::block_id.eq(block_id));
    let source = schema::masternode_lists::dsl::masternode_lists.filter(predicate);
    let values = (schema::masternode_lists::masternodes_merkle_root.eq(masternodes_root),
                  schema::masternode_lists::quorums_merkle_root.eq(quorums_root));
    diesel::update(source)
        .set(values)
        .execute(pooled_conn.deref_mut())
}

pub fn masternode_list_for_block(chain_id: i32, block_id: i32) -> QueryResult<MasternodeList> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = schema::masternode_lists::chain_id.eq(chain_id)
        .and(schema::masternode_lists::block_id.eq(block_id));
    schema::masternode_lists::dsl::masternode_lists::select(schema::masternode_lists::dsl::masternode_lists, schema::masternode_lists::dsl::masternode_lists::all_columns())
        .filter(predicate)
        .first::<MasternodeList>(pooled_conn.deref_mut())
}

pub fn masternode_lists_for_chain(chain_id: i32) -> QueryResult<Vec<MasternodeList>> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = schema::masternode_lists::chain_id.eq(chain_id);
    schema::masternode_lists::dsl::masternode_lists::select(schema::masternode_lists::dsl::masternode_lists, schema::masternode_lists::dsl::masternode_lists::all_columns())
        .filter(predicate)
        .get_results(pooled_conn.deref_mut())
}

// "block.height < %@ && block.blockHash IN %@ && (block.usedByQuorums.@count == 0)";


///
/// Masternode methods
///

pub fn create_masternode<'a>(
    chain_id: i32,
    address: i64,
    port: i16,
    core_last_connection_date: Option<NaiveDateTime>,
    core_protocol: i64,
    core_version: Option<&'a str>,
    is_valid: bool,
    platform_ping: i64,
    platform_ping_date: Option<NaiveDateTime>,
    platform_version: Option<&'a str>,
    known_confirmed_at_height: i32,
    update_height: i32,
    local_masternode_id: Option<i32>,
    prev_operator_bls_public_keys: BDictionary<'a, UInt256, UInt384>,
    prev_masternode_entry_hashes: BDictionary<'a, UInt256, UInt256>,
    prev_validity: BDictionary<'a, UInt256, Boolean>,
    confirmed_hash: UInt256,
    ipv6_address: UInt128,
    key_id_voting: UInt160,
    operator_bls_public_key: UInt384,
    provider_registration_transaction_hash: UInt256,
    masternode_entry_hash: UInt256,
) -> QueryResult<usize> {
    let mut pooled_conn = get_pooled_connection();
    let records = NewMasternode {
        chain_id,
        address,
        port,
        core_last_connection_date,
        core_protocol,
        core_version,
        is_valid,
        platform_ping,
        platform_ping_date,
        platform_version,
        known_confirmed_at_height,
        update_height,
        local_masternode_id,
        prev_operator_bls_public_keys,
        prev_masternode_entry_hashes,
        prev_validity,
        confirmed_hash,
        ipv6_address,
        key_id_voting,
        operator_bls_public_key,
        provider_registration_transaction_hash,
        masternode_entry_hash
    };
    diesel::insert_into(schema::masternodes::dsl::masternodes)
        .values(&records)
        .execute(pooled_conn.deref_mut())
}

//"block.chain == %@ && masternodes.@count == 0"
//NSArray *matchingMasternodeEntities = [DSSimplifiedMasternodeEntryEntity objectsInContext:self.managedObjectContext matching:@"utxoHash == %@ && utxoIndex == %@", masternodeHashData, @(governanceVote.masternodeUTXO.n)];
//DSSimplifiedMasternodeEntryEntity *simplifiedMasternodeEntryEntity = [DSSimplifiedMasternodeEntryEntity anyObjectInContext:self.managedObjectContext matching:@"providerRegistrationTransactionHash == %@", uint256_data(localMasternode.providerRegistrationTransaction.txHash)];

/**
 * "(providerRegistrationTransactionHash == %@) && (chain == %@)"
 */
pub fn read_masternode<'a, Predicate>(predicate: Predicate) -> QueryResult<Masternode<'a>>
    where Predicate:
        diesel::Expression<SqlType = diesel::sql_types::Bool> +
        diesel::expression::NonAggregate +
        diesel::expression::AppearsOnTable<schema::masternodes::dsl::masternodes> +
        diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite> +
        diesel::query_builder::QueryId {
    let mut pooled_conn = get_pooled_connection();
    schema::masternodes::dsl::masternodes::select(schema::masternodes::dsl::masternodes, schema::masternodes::dsl::masternodes::all_columns())
        .filter(predicate)
        .first::<Masternode>(pooled_conn.deref_mut())
}

/**
 * "(providerRegistrationTransactionHash == %@) && (chain == %@)"
 */
pub fn masternode_with_pro_reg_tx_hash<'a>(chain_id: i32, pro_reg_tx_hash: UInt256) -> QueryResult<Masternode<'a>> {
    let predicate = schema::masternodes::chain_id.eq(chain_id)
        .and(schema::masternodes::provider_registration_transaction_hash.eq(pro_reg_tx_hash));
    read_masternode(predicate)
}

/**
 * "(simplifiedMasternodeEntryHash == %@) && (chain == %@)"
 */
pub fn masternode_with_entry_hash<'a>(chain_id: i32, entry_hash: UInt256) -> QueryResult<Masternode<'a>> {
    let predicate = schema::masternodes::chain_id.eq(chain_id)
        .and(schema::masternodes::masternode_entry_hash.eq(entry_hash));
    read_masternode(predicate)
}


pub fn delete_masternodes(chain_id: i32) -> QueryResult<usize> {
    let predicate = schema::masternodes::chain_id.eq(chain_id);
    let source = schema::masternodes::dsl::masternodes.filter(predicate);
    delete(source)
}

pub fn delete_masternodes_having_provider_transaction_hashes(chain_id: i32, hashes: Vec<UInt256>) -> QueryResult<usize> {
    // "(chain == %@) && (providerRegistrationTransactionHash IN %@)"
    let predicate = schema::masternodes::chain_id.eq(chain_id)
        .and(schema::masternodes::provider_registration_transaction_hash.eq_any(hashes));
    let source = schema::masternodes::dsl::masternodes.filter(predicate);
    delete(source)
}

///
/// LocalMasternode methods
///
/// Usable queries:
/// "providerRegistrationTransaction.transactionHash.txHash == %@"
/// "providerRegistrationTransaction.transactionHash.chain == %@"
/// "block.chain == %@ && block.blockHash == %@"
/// "(providerRegistrationTransaction.transactionHash.txHash IN %@)"

// pub fn delete_local_masternodes(chain_id: i32) -> QueryResult<usize> {
//     // need to impl pro_reg_tx and it's join
//     let predicate = schema::local_masternodes::chain_id.eq(chain_id);
//     let source = local_masternodes.filter(predicate);
//     delete(source)
// }


// pub fn local_masternode_for_pro_reg_tx_hash(hash: UInt256) -> QueryResult<LocalMasternode> {
//     let mut pooled_conn = get_pooled_connection();
//     // let predicate  = schema::local_masternodes::provider_registration_transaction
//     local_masternodes::select(local_masternodes, local_masternodes::all_columns())
//         .filter(predicate)
//         .first::<LocalMasternode>(pooled_conn.deref_mut())
// }

///
/// Quorum methods
///
/// Usable queries
/// "chain == %@ && block.height == %@"
/// "chain == %@"
/// "quorumHashData == %@ && llmqType == %@ "
/// "(chain == %@) && (quorumEntryHash == %@)"
/// NSSortDescriptor *quorumTypeSortDescriptor = [[NSSortDescriptor alloc] initWithKey:@"llmqType" ascending:NO];
/// NSSortDescriptor *quorumHeightSortDescriptor = [[NSSortDescriptor alloc] initWithKey:@"block.height" ascending:NO];
/// NSSortDescriptor *quorumHashDataSortDescriptor = [[NSSortDescriptor alloc] initWithKey:@"quorumHashData" ascending:NO];
/// NSArray *sortDescriptors = @[quorumTypeSortDescriptor, quorumHeightSortDescriptor, quorumHashDataSortDescriptor];

pub fn create_quorum(
    block_id: i32,
    chain_id: i32,
    verified: bool,
    version: i16,
    all_commitment_aggregated_signature: UInt768,
    commitment_hash: UInt256,
    commitment_transaction_id: Option<i32>,
    quorum_index: Option<i32>,
    quorum_type: i16,
    quorum_hash: UInt256,
    quorum_public_key: UInt384,
    quorum_threshold_signature: UInt768,
    quorum_verification_vector_hash: UInt256,
    signers_count: i32,
    signers_bitset: Vec<u8>,
    valid_members_count: i32,
    valid_members_bitset: Vec<u8>,
) -> QueryResult<usize> {

    let data = NewQuorum {
        block_id,
        chain_id,
        verified,
        version,
        all_commitment_aggregated_signature,
        commitment_hash,
        commitment_transaction_id,
        quorum_index,
        quorum_type,
        quorum_hash,
        quorum_public_key,
        quorum_threshold_signature,
        quorum_verification_vector_hash,
        signers_count,
        signers_bitset,
        valid_members_count,
        valid_members_bitset
    };
    create(schema::quorums::dsl::quorums, &data)
}

pub fn delete_quorums(chain_id: i32) -> QueryResult<usize> {
    let predicate = schema::quorums::chain_id.eq(chain_id);
    let source = schema::quorums::dsl::quorums.filter(predicate);
    delete(source)
}

pub fn delete_quorums_having_hashes(chain_id: i32, hashes: Vec<UInt256>) -> QueryResult<usize> {
    let predicate = schema::quorums::chain_id.eq(chain_id)
        .and(schema::quorums::quorum_hash.eq_any(hashes));
    let source = schema::quorums::dsl::quorums.filter(predicate);
    delete(source)
}

pub fn quorum_for_commitment_hash<'a>(chain_id: i32, commitment_hash: UInt256) -> QueryResult<Quorum> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = schema::quorums::chain_id.eq(chain_id)
        .and(schema::quorums::commitment_hash.eq(commitment_hash));
    schema::quorums::dsl::quorums::select(schema::quorums::dsl::quorums, schema::quorums::dsl::quorums::all_columns())
        .filter(predicate)
        .first::<Quorum>(pooled_conn.deref_mut())
}
