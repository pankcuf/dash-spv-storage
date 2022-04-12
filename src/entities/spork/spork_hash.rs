use crate::entities::chain::chain::Chain;
use crate::entities::spork::spork::Spork;

#[derive(Debug)]
pub struct SporkHash<'a> {
    pub spork_hash: &'a [u8],

    pub chain: Chain<'a>,
    pub spork: Spork<'a>,
}
