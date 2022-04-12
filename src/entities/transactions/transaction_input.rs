use crate::entities::address::Address;
use crate::entities::transactions::transaction::Transaction;
use crate::entities::transactions::transaction_output::TransactionOutput;

#[derive(Debug)]
pub struct TransactionInput<'a> {
    pub n: i32,
    pub sequence: i32,
    pub signature: &'a [u8],
    pub tx_hash: &'a [u8],

    pub local_address: Address<'a>,
    pub prev_output: TransactionOutput<'a>,
    pub transaction: Transaction<'a>,
}
