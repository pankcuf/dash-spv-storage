use crate::entities::governance::object_hash::ObjectHash;
use crate::entities::governance::vote_hash::VoteHash;

pub struct Object<'a> {
    pub amount: i64,
    pub collateral_hash: &'a [u8],
    pub end_epoch: i64,
    pub id: &'a str,
    pub object_type: i32,
    pub parent_hash: &'a [u8],
    pub payment_address: &'a str,
    pub revision: i32,
    pub signature: &'a [u8],
    pub start_epoch: i64,
    pub timestamp: i64,
    pub total_votes_count: i64,
    pub url: &'a str,

    pub governance_object_hash: ObjectHash<'a>,
    pub vote_hashes: Vec<VoteHash<'a>>,
}
