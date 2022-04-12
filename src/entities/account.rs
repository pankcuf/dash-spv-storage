use crate::entities::chain::chain::Chain;
use crate::entities::derivation_path::DerivationPath;
use crate::entities::friend_request::FriendRequest;
use crate::entities::transactions::transaction_output::TransactionOutput;

#[derive(Debug)]
pub struct Account<'a> {
    pub index: i32,
    pub wallet_unique_id: &'a str,

    pub chain_id: &'a str,

    pub chain: Chain<'a>,
    pub derivation_paths: Vec<DerivationPath<'a>>,
    pub friend_requests: Vec<FriendRequest<'a>>,
    pub transaction_outputs: Vec<TransactionOutput<'a>>,
}
