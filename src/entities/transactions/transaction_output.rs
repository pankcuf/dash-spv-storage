use crate::entities::account::Account;
use crate::entities::address::Address;
use crate::entities::transactions::transaction::Transaction;
use crate::entities::transactions::transaction_input::TransactionInput;

#[derive(Debug)]
pub struct TransactionOutput<'a> {
    pub address: &'a [u8],
    pub n: i32,
    pub script: &'a [u8],
    pub shapeshift_outbound_address: &'a str,
    pub tx_hash: &'a [u8],
    pub value: i64,

    pub account: Account<'a>,
    pub local_address: Address<'a>,
    pub spent_in_input: TransactionInput<'a>,
    pub transaction: Transaction<'a>,
}
