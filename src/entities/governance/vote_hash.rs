use crate::entities::chain::chain::Chain;
use crate::entities::governance::object::Object;
use crate::entities::governance::vote::Vote;

pub struct VoteHash<'a> {
    pub vote_hash: &'a [u8],
    pub timestamp: i64,

    pub chain: Chain<'a>,
    pub governance_object: Object<'a>,
    pub governance_vote: Vote<'a>,
}
