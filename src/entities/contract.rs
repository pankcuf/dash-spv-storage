use crate::entities::chain::chain::Chain;
use crate::entities::identity::blockchain_identity::BlockchainIdentity;

#[derive(Debug)]
pub struct Contract<'a> {
    pub entropy: &'a [u8],
    pub local_contract_id: &'a str,
    pub registered_blockchain_identity_unique_id: &'a [u8],
    pub state: i16,

    pub chain: Chain<'a>,
    pub creator: BlockchainIdentity<'a>,
}
