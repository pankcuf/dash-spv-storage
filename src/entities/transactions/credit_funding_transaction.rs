use crate::entities::identity::blockchain_identity::BlockchainIdentity;
use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct CreditFundingTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub registered_blockchain_identity: BlockchainIdentity<'a>,
    pub topped_up_blockchain_identity: BlockchainIdentity<'a>,
}
