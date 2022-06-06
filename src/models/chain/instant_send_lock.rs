use dash_spv_primitives::crypto::{Boolean, UInt768};
use crate::schema::instant_send_locks;

/// "transaction.transactionHash.txHash == %@"
///
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct InstantSendLock {
    pub id: i32,
    pub verified: Boolean,
    pub signature: UInt768,
    pub quorum_id: Option<i32>,
    pub transaction_id: i32,
    // pub transaction_z27: i32,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="instant_send_locks"]
pub struct NewInstantSendLock {
    pub verified: Boolean,
    pub signature: UInt768,
    pub quorum_id: Option<i32>,
    pub transaction_id: i32,
}
