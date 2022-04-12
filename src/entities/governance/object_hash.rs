use crate::entities::chain::chain::Chain;
use crate::entities::governance::object::Object;

pub struct ObjectHash<'a> {
    pub governance_object_hash: &'a [u8],
    pub timestamp: i64,

    pub chain: Chain<'a>,
    pub governance_object: Object<'a>,
}
