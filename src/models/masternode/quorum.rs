use std::ops::DerefMut;
use dash_spv_models::common::LLMQType;
use dash_spv_models::masternode::LLMQEntry;
use dash_spv_primitives::consensus::encode;
use dash_spv_primitives::crypto::{UInt256, UInt384, UInt768};
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, Table};
use crate::connection_manager::get_pooled_connection;
use crate::{create, delete};
use crate::schema::quorums;

/// queries:
/// "chain == %@ && block.height == %@"
/// "chain == %@"
/// "quorumHashData == %@ && llmqType == %@"
/// "chain == %@ && quorumEntryHash== %@"
/// indexation:
/// ["llmqType": DESC, block.height": DESC, "quorumHashData": DESC]

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Quorum {
    pub id: i32,
    pub block_id: i32,
    pub chain_id: i32,
    pub verified: bool,
    pub version: i16,
    pub all_commitment_aggregated_signature: UInt768,
    pub commitment_hash: UInt256,
    pub commitment_transaction_id: Option<i32>,
    pub quorum_index: Option<i32>,
    pub quorum_type: i16,
    pub quorum_hash: UInt256,
    pub quorum_public_key: UInt384,
    pub quorum_threshold_signature: UInt768,
    pub quorum_verification_vector_hash: UInt256,
    pub signers_count: i32,
    pub signers_bitset: Vec<u8>,
    pub valid_members_count: i32,
    pub valid_members_bitset: Vec<u8>,
    // pub instant_send_locks: HashSet<i32>,
    // pub chain_locks: HashSet<i32>,
    // pub referenced_by_masternode_lists: HashSet<i32>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="quorums"]
pub struct NewQuorum {
    pub block_id: i32,
    pub chain_id: i32,
    pub verified: bool,
    pub version: i16,
    pub all_commitment_aggregated_signature: UInt768,
    pub commitment_hash: UInt256,
    pub commitment_transaction_id: Option<i32>,
    pub quorum_index: Option<i32>,
    pub quorum_type: i16,
    pub quorum_hash: UInt256,
    pub quorum_public_key: UInt384,
    pub quorum_threshold_signature: UInt768,
    pub quorum_verification_vector_hash: UInt256,
    pub signers_count: i32,
    pub signers_bitset: Vec<u8>,
    pub valid_members_count: i32,
    pub valid_members_bitset: Vec<u8>,
}

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
    create(quorums::dsl::quorums, &data)
}


pub fn delete_quorums(chain_id: i32) -> QueryResult<usize> {
    let predicate = quorums::chain_id.eq(chain_id);
    let source = quorums::dsl::quorums.filter(predicate);
    delete(source)
}

pub fn delete_quorums_having_hashes(chain_id: i32, hashes: Vec<UInt256>) -> QueryResult<usize> {
    let predicate = quorums::chain_id.eq(chain_id)
        .and(quorums::quorum_hash.eq_any(hashes));
    let source = quorums::dsl::quorums.filter(predicate);
    delete(source)
}

/// "chain == %@ && SUBQUERY(referencedByMasternodeLists, $masternodeList, $masternodeList.block.height > %@).@count == 0", self.context.chain_id, oldest_block_height;
pub fn delete_quorums_since_height(chain_id: i32, block_height: u32) -> QueryResult<usize> {
    let predicate = quorums::chain_id.eq(chain_id);
    // TODO: impl joins
    let source = quorums::dsl::quorums.filter(predicate);
    delete(source)
}

pub fn quorum_for_commitment_hash<'a, 'b>(chain_id: i32, commitment_hash: UInt256) -> QueryResult<Quorum> {
    let mut pooled_conn = get_pooled_connection();
    let predicate = quorums::chain_id.eq(chain_id)
        .and(quorums::commitment_hash.eq(commitment_hash));
    quorums::dsl::quorums.select(quorums::dsl::quorums::all_columns())
        .filter(predicate)
        .first::<Quorum>(pooled_conn.deref_mut())
}

/// "chain == %@ && SUBQUERY(referencedByMasternodeLists, $masternodeList, $masternodeList.block.height > %@).@count == 0", self.context.chain_id, oldest_block_height;
pub fn quorums_since_height(chain_id: i32, block_height: u32) -> QueryResult<Vec<Quorum>> {
    let mut pooled_conn = get_pooled_connection();
    // TODO: impl joins
    let predicate = quorums::chain_id.eq(chain_id);
    quorums::dsl::quorums.select(quorums::dsl::quorums::all_columns())
        .filter(predicate)
        .get_results::<Quorum>(pooled_conn.deref_mut())
}


impl Quorum {
    pub fn to_model(&self) -> LLMQEntry {
        LLMQEntry {
            version: self.version as u16,
            llmq_hash: self.quorum_hash,
            index: match self.quorum_index {
                Some(i) => Some(i as u32),
                None => None
            },
            public_key: self.quorum_public_key,
            threshold_signature: self.quorum_threshold_signature,
            verification_vector_hash: self.quorum_verification_vector_hash,
            all_commitment_aggregated_signature: self.all_commitment_aggregated_signature,
            signers_count: encode::VarInt(self.signers_count as u64),
            llmq_type: LLMQType::from(self.quorum_type as u8),
            valid_members_count: encode::VarInt(self.valid_members_count as u64),
            signers_bitset: self.signers_bitset.clone(),
            valid_members_bitset: self.valid_members_bitset.clone(),
            length: 0 /*offset*/,
            entry_hash: self.commitment_hash,
            verified: self.version > 0/* self.verified*/,
            saved: true,
            commitment_hash: None
        }
    }



}
