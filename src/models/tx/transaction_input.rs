use dash_spv_primitives::crypto::UInt256;
use crate::schema::transaction_inputs;

/// queries:
/// "localAddress.derivationPath.friendRequest == %@"
/// indexation:
/// "transaction.transactionHash.blockHeight": ASC
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct TransactionInput {
    pub id: i32,

    pub local_address_id: Option<i32>,
    pub prev_output_id: Option<i32>,
    pub transaction_id: i32,
    // pub transaction_27: i32,
    // pub transaction_fok: i32,

    pub n: i32,
    pub sequence: i32,
    pub signature: Vec<u8>,
    pub tx_hash: UInt256,

}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="transaction_inputs"]
pub struct NewTransactionInput {
    pub local_address_id: Option<i32>,
    pub prev_output_id: Option<i32>,
    pub transaction_id: i32,
    // pub transaction_27: i32,
    // pub transaction_fok: i32,

    pub n: i32,
    pub sequence: i32,
    pub signature: Vec<u8>,
    pub tx_hash: UInt256,
}
