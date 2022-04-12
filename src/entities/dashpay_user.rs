use crate::entities::chain::chain::Chain;
use crate::entities::friend_request::FriendRequest;
use crate::entities::identity::blockchain_identity::BlockchainIdentity;

#[derive(Debug)]
pub struct DashpayUser<'a> {
    pub avatar_fingerprint: &'a [u8],
    pub avatar_hash: &'a [u8],
    pub avatar_path: &'a str,
    pub created_at: i64,
    pub display_name: &'a str,
    pub document_id: &'a str,
    pub local_profile_document_revision: i32,
    pub original_entropy_data: &'a [u8],
    pub public_message: &'a str,
    pub remote_profile_document_revision: i32,
    pub updated_at: i64,

    pub associated_blockchain_identity: BlockchainIdentity<'a>,
    pub chain: Chain<'a>,
    pub friends: Vec<DashpayUser<'a>>,
    pub incoming_requests: Vec<FriendRequest<'a>>,
    pub outgoing_requests: Vec<FriendRequest<'a>>,
}
