use crate::entities::chain::chain::Chain;
use crate::entities::contract::Contract;
use crate::entities::dashpay_user::DashpayUser;
use crate::entities::identity::blockchain_identity_key_path::BlockchainIdentityKeyPath;
use crate::entities::identity::blockchain_identity_username::BlockchainIdentityUsername;
use crate::entities::identity::blockchain_invitation::BlockchainInvitation;
use crate::entities::transactions::credit_funding_transaction::CreditFundingTransaction;

#[derive(Debug)]
pub struct BlockchainIdentity<'a> {
    pub credit_balance: i64,
    pub dashpay_sync_block_hash: &'a [u8],
    pub is_local: bool,
    pub last_checked_incoming_contacts_timestamp: i64,
    pub last_checked_outgoing_contacts_timestamp: i64,
    pub last_checked_profile_timestamp: i64,
    pub last_checked_usernames_timestamp: i64,
    pub registration_status: i16,
    pub unique_id: &'a [u8],

    pub associated_invitation: BlockchainInvitation<'a>,
    pub chain: Chain<'a>,
    pub created_contracts: Vec<Contract<'a>>,
    pub dashpay_username: BlockchainIdentityUsername<'a>,
    pub key_paths: Vec<BlockchainIdentityKeyPath<'a>>,
    pub matching_dashpay_user: DashpayUser<'a>,
    pub registration_funding_transaction: CreditFundingTransaction,
    pub top_up_funding_transactions: Vec<CreditFundingTransaction>,
    pub usernames: Vec<BlockchainIdentityUsername<'a>>,

}
