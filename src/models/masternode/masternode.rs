use std::ops::DerefMut;
use chrono::NaiveDateTime;
use dash_spv_models::{common, masternode};
use dash_spv_models::common::BlockData;
use dash_spv_primitives::crypto::{BDictionary, Boolean, UInt128, UInt160, UInt256, UInt384};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};

use crate::delete;
use crate::connection_manager::get_pooled_connection;
use crate::schema::masternodes;
/// queries
/// "chain == %@"
/// "(((address >> %@) & 255) == %@)"
/// "ANY masternodeLists.block.height == %@"
/// "masternodeLists.@count == 0"
/// "utxoHash == %@ && utxoIndex == %@" ????
/// "providerRegistrationTransactionHash == %@"
#[derive(Identifiable, Queryable, Associations, PartialEq, Eq, Debug)]
// #[belongs_to(LocalMasternode)]
pub struct Masternode<'a> {
    pub id: i32,
    pub chain_id: i32,
    pub address: i64,
    pub port: i16,
    pub core_last_connection_date: Option<NaiveDateTime>,
    pub core_protocol: i64,
    pub core_version: Option<String>,
    pub is_valid: bool,
    pub platform_ping: i64,
    pub platform_ping_date: Option<NaiveDateTime>,
    pub platform_version: Option<String>,
    pub known_confirmed_at_height: i32,
    pub update_height: i32,
    pub local_masternode_id: Option<i32>,
    pub prev_operator_bls_public_keys: BDictionary<'a, UInt256, UInt384>,
    pub prev_masternode_entry_hashes: BDictionary<'a, UInt256, UInt256>,
    pub prev_validity: BDictionary<'a, UInt256, Boolean>,
    pub confirmed_hash: UInt256,
    pub ipv6_address: UInt128,
    pub key_id_voting: UInt160,
    pub operator_bls_public_key: UInt384,
    pub provider_registration_transaction_hash: UInt256,
    pub masternode_entry_hash: UInt256,

    // pub addresses: BTreeSet<i32>,
    // pub governance_votes: HashSet<i32>,
    // pub masternode_lists: HashSet<i32>,
}

#[derive(Insertable, Associations, PartialEq, Eq, Debug)]
#[table_name="masternodes"]
pub struct NewMasternode<'a> {
    pub chain_id: i32,
    pub address: i64,
    pub port: i16,
    pub core_last_connection_date: Option<NaiveDateTime>,
    pub core_protocol: i64,
    pub core_version: Option<&'a str>,
    pub is_valid: bool,
    pub platform_ping: i64,
    pub platform_ping_date: Option<NaiveDateTime>,
    pub platform_version: Option<&'a str>,
    pub known_confirmed_at_height: i32,
    pub update_height: i32,
    pub local_masternode_id: Option<i32>,
    pub prev_operator_bls_public_keys: BDictionary<'a, UInt256, UInt384>,
    pub prev_masternode_entry_hashes: BDictionary<'a, UInt256, UInt256>,
    pub prev_validity: BDictionary<'a, UInt256, Boolean>,
    pub confirmed_hash: UInt256,
    pub ipv6_address: UInt128,
    pub key_id_voting: UInt160,
    pub operator_bls_public_key: UInt384,
    pub provider_registration_transaction_hash: UInt256,
    pub masternode_entry_hash: UInt256,
}


//"block.chain == %@ && masternodes.@count == 0"
//NSArray *matchingMasternodeEntities = [DSSimplifiedMasternodeEntryEntity objectsInContext:self.managedObjectContext matching:@"utxoHash == %@ && utxoIndex == %@", masternodeHashData, @(governanceVote.masternodeUTXO.n)];
//DSSimplifiedMasternodeEntryEntity *simplifiedMasternodeEntryEntity = [DSSimplifiedMasternodeEntryEntity anyObjectInContext:self.managedObjectContext matching:@"providerRegistrationTransactionHash == %@", uint256_data(localMasternode.providerRegistrationTransaction.txHash)];

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
    diesel::insert_into(masternodes::dsl::masternodes)
        .values(&records)
        .execute(pooled_conn.deref_mut())
}


/// "(providerRegistrationTransactionHash == %@) && (chain == %@)"
pub fn read_masternode<'a, Predicate>(predicate: Predicate) -> QueryResult<Masternode<'a>>
    where Predicate:
    diesel::Expression<SqlType = diesel::sql_types::Bool> +
    diesel::expression::NonAggregate +
    diesel::expression::AppearsOnTable<masternodes::dsl::masternodes> +
    diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite> +
    diesel::query_builder::QueryId {
    let mut pooled_conn = get_pooled_connection();
    let selection = masternodes::dsl::masternodes::all_columns();
    let ss = masternodes::dsl::masternodes.select(selection);
    let filtered = ss.filter(predicate);
    filtered
        .first::<Masternode>(pooled_conn.deref_mut())
}

/// "(providerRegistrationTransactionHash == %@) && (chain == %@)"
pub fn masternode_with_pro_reg_tx_hash<'a>(chain_id: i32, pro_reg_tx_hash: UInt256) -> QueryResult<Masternode<'a>> {
    let predicate = masternodes::chain_id.eq(chain_id)
        .and(masternodes::provider_registration_transaction_hash.eq(pro_reg_tx_hash));
    read_masternode(predicate)
}

/// "(simplifiedMasternodeEntryHash == %@) && (chain == %@)"
pub fn masternode_with_entry_hash<'a>(chain_id: i32, entry_hash: UInt256) -> QueryResult<Masternode<'a>> {
    let predicate = masternodes::chain_id.eq(chain_id)
        .and(masternodes::masternode_entry_hash.eq(entry_hash));
    read_masternode(predicate)
}


pub fn delete_masternodes(chain_id: i32) -> QueryResult<usize> {
    let predicate = masternodes::chain_id.eq(chain_id);
    let source = masternodes::dsl::masternodes.filter(predicate);
    delete(source)
}

/// "(chain == %@) && (providerRegistrationTransactionHash IN %@)"
pub fn delete_having_provider_transaction_hashes(chain_id: i32, hashes: Vec<UInt256>) -> QueryResult<usize> {
    let predicate = masternodes::chain_id.eq(chain_id)
        .and(masternodes::provider_registration_transaction_hash.eq_any(hashes));
    let source = masternodes::dsl::masternodes.filter(predicate);
    delete(source)
}

/// "masternodeLists.@count == 0"
pub fn delete_masternodes_with_empty_lists(chain_id: i32) -> QueryResult<usize> {
    let predicate = masternodes::chain_id.eq(chain_id);
    let source = masternodes::dsl::masternodes.filter(predicate);
    delete(source)
}


impl<'a> Masternode<'a> {
    pub fn simplified_masternode_entry_with_block_height_lookup<BHL>(&self, block_height_lookup: BHL) -> masternode::MasternodeEntry
        where BHL: Fn(UInt256) -> u32 {
        // chain ???
        // confirmed_hash_hashed_with_provider_registration_transaction_hash ???
        // block data ???
        // let block_height = block_height_lookup(self.)


        let previous_operator_public_keys = self.prev_operator_bls_public_keys.map
            .iter()
            .map(|(&hash, &value)|(BlockData { height: block_height_lookup(hash), hash }, value))
            .collect();

        let previous_entry_hashes = self.prev_masternode_entry_hashes.map
            .iter()
            .map(|(&hash, &value)|(BlockData { height: block_height_lookup(hash), hash }, value))
            .collect();
        let previous_validity = self.prev_validity.map
            .iter()
            .map(|(&hash, &value)|(BlockData { height: block_height_lookup(hash), hash }, value.0))
            .collect();



        masternode::MasternodeEntry {
            provider_registration_transaction_hash: self.provider_registration_transaction_hash,
            confirmed_hash: self.confirmed_hash,
            confirmed_hash_hashed_with_provider_registration_transaction_hash: None,
            socket_address: common::SocketAddress {
                ip_address: self.ipv6_address,
                port: self.port as u16
            },
            operator_public_key: self.operator_bls_public_key,
            previous_operator_public_keys,
            previous_entry_hashes,
            previous_validity,
            known_confirmed_at_height: Some(self.known_confirmed_at_height as u32),
            update_height: self.update_height as u32,
            key_id_voting: self.key_id_voting,
            is_valid: self.is_valid,
            entry_hash: self.masternode_entry_hash
        }
    }

}
