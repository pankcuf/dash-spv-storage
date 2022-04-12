use crate::entities::derivation_path::DerivationPath;
use crate::entities::identity::blockchain_identity::BlockchainIdentity;

#[derive(Debug)]
pub struct BlockchainIdentityKeyPath<'a> {
    pub key_id: i32,
    pub key_status: i16,
    pub key_type: i16,
    pub path: Vec<&'a [u8]>,
    pub public_key: &'a [u8],

    pub blockchain_identity: BlockchainIdentity<'a>,
    pub derivation_path: DerivationPath<'a>,
}
