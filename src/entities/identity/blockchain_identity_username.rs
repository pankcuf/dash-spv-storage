use crate::entities::identity::blockchain_identity::BlockchainIdentity;

#[derive(Debug)]
pub struct BlockchainIdentityUsername<'a> {
    pub domain: &'a str,
    pub salt: &'a [u8],
    pub status: i16,
    pub string_value: &'a str,

    pub blockchain_identity: BlockchainIdentity<'a>,
    pub blockchain_identity_used_for_dashpay: BlockchainIdentity<'a>,

}
