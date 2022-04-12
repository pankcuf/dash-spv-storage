use crate::entities::address::Address;
use crate::entities::chain::chain::Chain;
use crate::entities::governance::vote::Vote;
use crate::entities::masternode::local_masternode::LocalMasternode;
use crate::entities::masternode::masternode_list::MasternodeList;

#[derive(Debug)]
pub struct MasternodeEntry<'a> {
    pub address: i64,
    pub confirmed_hash: &'a [u8],
    pub core_last_connection_date: i64,
    pub core_protocol: i64,
    pub core_version: &'a str,
    pub ipv6_address: &'a str,
    pub is_valid: bool,
    pub key_id_voting: &'a str,
    pub known_confirmed_at_height: i32,
    pub operator_bls_public_key: &'a [u8],
    pub platform_ping: i64,
    pub platform_ping_date: i64,
    pub platform_version: &'a str,
    pub port: i16,
    pub previous_operator_bls_public_keys: Vec<&'a [u8]>,
    pub previous_simplified_masternode_entry_hashes: Vec<&'a [u8]>,
    pub previous_validity: Vec<&'a [u8]>,
    pub provider_registration_transaction_hash: &'a [u8],
    pub masternode_entry_hash: &'a [u8],
    pub update_height: i32,

    pub addresses: Vec<Address<'a>>,
    pub chain: Chain<'a>,
    pub governance_votes: Vec<Vote<'a>>,
    pub local_masternode: LocalMasternode<'a>,
    pub masternode_lists: Vec<MasternodeList<'a>>,
}
