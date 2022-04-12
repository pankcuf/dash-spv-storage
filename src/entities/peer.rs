use crate::entities::chain::chain::Chain;

#[derive(Debug)]
pub struct Peer<'a> {
    pub address: i32,
    pub last_requested_governance_sync: i64,
    pub last_requested_masternode_list: i64,
    pub low_preference_till: i64,
    pub misbehavin: i16,
    pub port: i16,
    pub priority: i32,
    pub services: i64,
    pub timestamp: i64,

    pub chain: Chain<'a>,
}
