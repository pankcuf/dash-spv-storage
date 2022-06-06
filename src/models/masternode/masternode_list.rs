use std::ops::DerefMut;
use dash_spv_primitives::crypto::UInt256;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};
use crate::{create, delete};
use crate::connection_manager::get_pooled_connection;
use crate::schema::masternode_lists;
// "block.height < %@ && block.blockHash IN %@ && (block.usedByQuorums.@count == 0)";

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct MasternodeList {
    pub id: i32,
    pub block_id: i32,
    pub chain_id: i32,
    pub masternodes_merkle_root: UInt256,
    pub quorums_merkle_root: UInt256,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="masternode_lists"]
pub struct NewMasternodeList {
    pub block_id: i32,
    pub chain_id: i32,
    pub masternodes_merkle_root: UInt256,
    pub quorums_merkle_root: UInt256,
}

pub fn create_masternode_list(block_id: i32,
                              chain_id: i32,
                              masternodes_merkle_root: UInt256,
                              quorums_merkle_root: UInt256) -> QueryResult<usize> {
    let target = masternode_lists::dsl::masternode_lists;
    let records = NewMasternodeList { block_id, chain_id, masternodes_merkle_root, quorums_merkle_root };
    create(target, &records)
}

pub fn delete_masternode_list(chain_id: i32, block_id: i32) -> QueryResult<usize> {
    let predicate = masternode_lists::chain_id.eq(chain_id)
        .and(masternode_lists::block_id.eq(block_id));
    let source = masternode_lists::dsl::masternode_lists.filter(predicate);
    delete(source)
}

pub fn delete_masterenode_lists_for_chain(chain_id: i32) -> QueryResult<usize> {
    let predicate = masternode_lists::chain_id.eq(chain_id);
    let source = masternode_lists::dsl::masternode_lists.filter(predicate);
    delete(source)
}

pub fn update_masternode_list(block_id: i32, chain_id: i32, masternodes_root: UInt256, quorums_root: UInt256) -> QueryResult<usize> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = masternode_lists::chain_id.eq(chain_id)
        .and(masternode_lists::block_id.eq(block_id));
    let source = masternode_lists::dsl::masternode_lists.filter(predicate);
    let values = (masternode_lists::masternodes_merkle_root.eq(masternodes_root),
                  masternode_lists::quorums_merkle_root.eq(quorums_root));
    diesel::update(source)
        .set(values)
        .execute(pooled_conn.deref_mut())
}

pub fn masternode_list_for_block(chain_id: i32, block_id: i32) -> QueryResult<MasternodeList> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = masternode_lists::chain_id.eq(chain_id)
        .and(masternode_lists::block_id.eq(block_id));
    masternode_lists::dsl::masternode_lists.select(masternode_lists::dsl::masternode_lists::all_columns())
        .filter(predicate)
        .first::<MasternodeList>(pooled_conn.deref_mut())
}

pub fn masternode_lists_for_chain(chain_id: i32) -> QueryResult<Vec<MasternodeList>> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = masternode_lists::chain_id.eq(chain_id);
    masternode_lists::dsl::masternode_lists.select(masternode_lists::dsl::masternode_lists::all_columns())
        .filter(predicate)
        .get_results(pooled_conn.deref_mut())
}
