use dash_spv_primitives::crypto::UInt256;
use crate::schema::coinbase_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct CoinbaseTransaction {
    pub id: i32,
    pub base_id: i32,
    pub height: i32,
    pub merkle_root_mn_list: UInt256,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="coinbase_transactions"]
pub struct NewCoinbaseTransaction {
    pub base_id: i32,
    pub height: i32,
    pub merkle_root_mn_list: UInt256,
}
