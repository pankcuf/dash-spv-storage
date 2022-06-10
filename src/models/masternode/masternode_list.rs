use std::collections::{BTreeMap, HashMap};
use std::ops::DerefMut;
use dash_spv_models::common::LLMQType;
use dash_spv_models::masternode::{LLMQEntry, MasternodeEntry};
use dash_spv_primitives::crypto::UInt256;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};
use crate::{create, delete, schema};
use crate::connection_manager::get_pooled_connection;
use crate::models::masternode::{Masternode, Quorum};
use crate::schema::masternode_lists;
use crate::schema::masternode_list_masternodes;
use crate::schema::masternode_list_quorums;
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

pub fn delete_masternode_lists(chain_id: i32) -> QueryResult<usize> {
    let predicate = masternode_lists::chain_id.eq(chain_id);
    let source = masternode_lists::dsl::masternode_lists.filter(predicate);
    delete(source)
}

/// "block.chain == %@ && masternodes.@count == 0"
// pub fn delete_empty_masternode_lists(chain_id: i32) -> QueryResult<usize> {
//     let predicate = masternode_lists::chain_id.eq(chain_id);
//     // let hardware = hardware::table
//     //     .left_join(units::table.on(hardware::unit_id.eq(units::id.nullable())))
//
//     //schema::masternode_list_masternodes::table::left_join()
// }

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

/// "block.height < %@ && block.blockHash IN %@ && (block.usedByQuorums.@count == 0)"
pub fn masternode_lists_in(chain_id: i32, block_ids: Vec<i32>) -> QueryResult<Vec<MasternodeList>> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = masternode_lists::chain_id.eq(chain_id)
        .and(masternode_lists::block_id.eq_any(block_ids));
    // TODO: make it normal

    masternode_lists::dsl::masternode_lists.select(masternode_lists::dsl::masternode_lists::all_columns())
        .filter(predicate)
        .get_results(pooled_conn.deref_mut())
}

impl<'a> MasternodeList {
    //+ (void)deleteAllOnChainEntity:(DSChainEntity *)chainEntity;

    pub fn masternode_list_with_simplified_masternode_entry_pool_and_lookup<BHL>(
        &self,
        masternodes: BTreeMap<UInt256, MasternodeEntry>,
        quorums: HashMap<LLMQType, HashMap<UInt256, LLMQEntry>>,
        block_height_lookup: BHL
    ) -> dash_spv_models::masternode::MasternodeList
        where BHL: Fn(UInt256) -> u32 + Copy
    {
        let mut pooled_conn = get_pooled_connection();
        let mut masternode_entries: BTreeMap<UInt256, MasternodeEntry> = BTreeMap::new();
        let mut quorum_entries: HashMap<LLMQType, HashMap<UInt256, LLMQEntry>> = HashMap::new();

        if let Ok(ids) = masternode_list_masternodes::dsl::masternode_list_masternodes
            .select(masternode_list_masternodes::masternode_id)
            .filter(masternode_list_masternodes::dsl::masternode_list_id.eq(self.id))
            .get_results::<i32>(pooled_conn.deref_mut()) {
            ids.iter().for_each(|&i| {
                if let Ok(result) = schema::masternodes::dsl::masternodes
                    .select(schema::masternodes::dsl::masternodes::all_columns())
                    .filter(schema::masternodes::dsl::id.eq(i))
                    .get_results::<Masternode>(pooled_conn.deref_mut()) {
                    result.iter().for_each(|entity| {
                        let hash = entity.provider_registration_transaction_hash;
                        if let Some(node) = masternodes.get(&hash) {
                            masternode_entries.insert(hash, node.clone());
                        } else {
                            masternode_entries.insert(hash, entity.simplified_masternode_entry_with_block_height_lookup(block_height_lookup));
                        }
                    });
                }
            });
        }
        if let Ok(ids) = masternode_list_quorums::dsl::masternode_list_quorums
            .select(masternode_list_quorums::quorum_id)
            .filter(masternode_list_quorums::dsl::masternode_list_id.eq(self.id))
            .get_results::<i32>(pooled_conn.deref_mut()) {
            ids.iter().for_each(|&i| {
                if let Ok(result) = schema::quorums::dsl::quorums
                    .select(schema::quorums::dsl::quorums::all_columns())
                    .filter(schema::quorums::dsl::id.eq(i))
                    .get_results::<Quorum>(pooled_conn.deref_mut()) {
                    result.iter().for_each(|entity| {
                        let llmq_type = LLMQType::from(entity.quorum_type as u8);
                        let llmq_hash = entity.quorum_hash;
                        quorum_entries.entry(llmq_type).or_insert(HashMap::new()).insert(llmq_hash.clone(), if let Some(quorums_of_type) = quorums.get(&llmq_type) {
                            if let Some(entry) = quorums_of_type.get(&llmq_hash) {
                                Some(entry.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }.unwrap_or(entity.to_model()));
                    });
                }
            });
        }



        dash_spv_models::masternode::MasternodeList {
            block_hash: Default::default(),
            known_height: u32::MAX,
            masternode_merkle_root: Some(self.masternodes_merkle_root),
            llmq_merkle_root: Some(self.quorums_merkle_root),
            masternodes: masternode_entries,
            quorums: quorum_entries
        }

        //[DSMasternodeList masternodeListWithSimplifiedMasternodeEntries:
        // masternodeEntriesArray quorumEntries:quorumEntriesArray
        // atBlockHash:self.block.blockHash.UInt256
        // atBlockHeight:self.block.height
        // withMasternodeMerkleRootHash:self.masternodeListMerkleRoot.UInt256
        // withQuorumMerkleRootHash:self.quorumListMerkleRoot.UInt256
        // onChain:self.block.chain.chain];

    }

}





