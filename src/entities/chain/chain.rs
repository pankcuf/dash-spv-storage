use crate::entities::account::Account;
use crate::entities::chain::chain_lock::ChainLock;
use crate::entities::contract::Contract;
use crate::entities::dashpay_user::DashpayUser;
use crate::entities::derivation_path::DerivationPath;
use crate::entities::governance::object_hash::ObjectHash;
use crate::entities::governance::vote::Vote;
use crate::entities::identity::blockchain_identity::BlockchainIdentity;
use crate::entities::identity::blockchain_invitation::BlockchainInvitation;
use crate::entities::masternode::masternode_entry::MasternodeEntry;
use crate::entities::masternode::quorum_entry::QuorumEntry;
use crate::entities::merkle_block::MerkleBlock;
use crate::entities::peer::Peer;
use crate::entities::spork::spork::Spork;
use crate::entities::transactions::transaction_hash::TransactionHash;

#[derive(Debug)]
pub struct Chain<'a> {
    pub base_block_hash: &'a [u8],
    pub checkpoints: &'a [u8],
    pub devnet_id: &'a str,
    pub sync_block_chain_work: &'a [u8],
    pub sync_block_hash: &'a [u8],
    pub sync_block_height: i32,
    pub sync_block_timestamp: i64,
    pub sync_locators: Vec<&'a [u8]>,
    pub total_governance_objects_count: i32,
    pub chain_type: i16,

    pub accounts: Vec<Account<'a>>,
    pub blocks: Vec<MerkleBlock<'a>>,
    pub contacts: Vec<DashpayUser<'a>>,
    pub contracts: Vec<Contract<'a>>,
    pub derivation_paths: Vec<DerivationPath<'a>>,
    pub governance_object_hashes: Vec<ObjectHash<'a>>,
    pub identities: Vec<BlockchainIdentity<'a>>,
    pub invitations: Vec<BlockchainInvitation<'a>>,
    pub last_chain_lock: ChainLock<'a>,
    pub peers: Vec<Peer<'a>>,
    pub quorums: Vec<QuorumEntry<'a>>,
    pub masternode_entries: Vec<MasternodeEntry<'a>>,
    pub sporks: Vec<Spork<'a>>,
    pub transaction_hashes: Vec<TransactionHash<'a>>,
    pub votes: Vec<Vote<'a>>,
}
