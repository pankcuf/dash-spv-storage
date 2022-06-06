use chrono::NaiveDateTime;
use crate::schema::peers;

/// queries:
/// "chain == %@"
/// "chain == %@ && (((address >> %@) & 255) == %@)"
/// "(chain == %@) && !(address in %@)"
/// "(chain == %@) && (address in %@)"
/// "address == %@ && port == %@"
/// indexation:
/// ["priority": DESC, "address": ASC]

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct Peer {
    pub id: i32,
    pub chain_id: i32,
    pub address: i32,
    pub port: i16,
    pub misbehavin: i16,
    pub priority: i32,
    pub services: i64,

    pub timestamp: NaiveDateTime,
    pub last_requested_governance_sync: NaiveDateTime,
    pub last_requested_masternode_list: NaiveDateTime,
    pub low_preference_till: NaiveDateTime,

}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="peers"]
pub struct NewPeer {
    pub chain_id: i32,
    pub address: i32,
    pub port: i16,
    pub misbehavin: i16,
    pub priority: i32,
    pub services: i64,

    pub timestamp: NaiveDateTime,
    pub last_requested_governance_sync: NaiveDateTime,
    pub last_requested_masternode_list: NaiveDateTime,
    pub low_preference_till: NaiveDateTime,
}
