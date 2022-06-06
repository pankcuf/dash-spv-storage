use dash_spv_primitives::crypto::UInt256;
use crate::schema::sporks;

/// queries:
/// (sporkhash) "(sporkHash.chain == %@)"
/// indexation:

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Spork {
    pub id: i32,
    pub chain_id: i32,
    pub identifier: i32,
    pub time_signed: i64,
    pub value: i64,
    pub spork_hash: UInt256,
    pub signature: Vec<u8>,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="sporks"]
pub struct NewSpork {
    pub chain_id: i32,
    pub identifier: i32,
    pub time_signed: i64,
    pub value: i64,

    pub spork_hash: UInt256,
    pub signature: Vec<u8>,
}
