use crate::entities::chain::chain::Chain;
use crate::entities::transactions::transaction::Transaction;

#[derive(Debug)]
pub struct TransactionHash<'a> {
    pub block_height: i32,
    pub timestamp: f64,
    pub tx_hash: &'a [u8],

    pub chain: Chain<'a>,
    pub transaction: Transaction<'a>,
}
