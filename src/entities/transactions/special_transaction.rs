use crate::entities::address::Address;
use crate::entities::transactions::transaction::Transaction;

#[derive(Debug)]
pub struct SpecialTransaction<'a> {
    pub base: Transaction<'a>,

    pub special_transaction_version: i16,

    pub addresses: Vec<Address<'a>>,
}
