use crate::entities::chain::chain::Chain;
use crate::entities::identity::blockchain_identity::BlockchainIdentity;

#[derive(Debug)]
pub struct BlockchainInvitation<'a> {
    pub link: &'a str,

    pub blockchain_identity: BlockchainIdentity<'a>,
    pub chain: Chain<'a>,
}
