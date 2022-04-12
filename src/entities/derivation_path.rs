use crate::entities::account::Account;
use crate::entities::address::Address;
use crate::entities::chain::chain::Chain;
use crate::entities::friend_request::FriendRequest;
use crate::entities::identity::blockchain_identity_key_path::BlockchainIdentityKeyPath;

#[derive(Debug)]
pub struct DerivationPath<'a> {
    pub derivation_path: &'a [u8],
    pub public_key_id: &'a str,
    pub sync_block_height: i32,

    pub account: Account<'a>,
    pub addresses: Vec<Address<'a>>,
    pub chain: Chain<'a>,
    pub friend_request: FriendRequest<'a>,
    pub identity_key_paths: Vec<BlockchainIdentityKeyPath<'a>>,
}
