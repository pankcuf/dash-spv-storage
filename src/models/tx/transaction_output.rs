use dash_spv_primitives::crypto::UInt256;
use crate::schema::transaction_outputs;

/// queries:
/// "txHash == %@ && n == %d"
/// "address == %@"
///
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct TransactionOutput {
    pub id: i32,

    pub account_id: Option<i32>,
    pub local_address_id: Option<i32>,
    pub spent_in_input_id: Option<i32>,
    pub transaction_id: i32,
    // pub transaction_27: i32,
    // pub transaction_fok: i32,

    pub address: String,
    pub shapeshift_outbound_address: Option<String>,
    pub n: i32,
    pub value: i64,
    pub script: Vec<u8>,
    pub tx_hash: UInt256,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="transaction_outputs"]
pub struct NewTransactionOutput<'a> {
    pub account_id: Option<i32>,
    pub local_address_id: Option<i32>,
    pub spent_in_input_id: Option<i32>,
    pub transaction_id: i32,
    // pub transaction_27: i32,
    // pub transaction_fok: i32,

    pub address: &'a str,
    pub shapeshift_outbound_address: Option<&'a str>,
    pub n: i32,
    pub value: i64,
    pub script: Vec<u8>,
    pub tx_hash: UInt256,

}
