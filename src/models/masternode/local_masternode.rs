use crate::schema::local_masternodes;
/// queries:
/// "providerRegistrationTransaction.transactionHash.txHash == %@"
/// "providerRegistrationTransaction.transactionHash.chain == %@"
/// "(providerRegistrationTransaction.transactionHash.txHash IN %@)"

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
// #[belongs_to(Masternode)]
pub struct LocalMasternode {
    pub id: i32,
    pub operator_keys_index: i32,
    pub operator_keys_wallet_id: String,
    pub owner_keys_index: i32,
    pub owner_keys_wallet_id: String,
    pub holding_keys_index: i32,
    pub voting_keys_wallet_id: String,
    pub holding_keys_wallet_id: String,
    pub voting_keys_index: i32,
    pub provider_registration_transaction_id: i32,
    pub masternode_id: i32,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="local_masternodes"]
pub struct NewLocalMasternode<'a> {
    pub operator_keys_index: i32,
    pub operator_keys_wallet_id: &'a str,
    pub owner_keys_index: i32,
    pub owner_keys_wallet_id: &'a str,
    pub holding_keys_index: i32,
    pub voting_keys_wallet_id: &'a str,
    pub holding_keys_wallet_id: &'a str,
    pub voting_keys_index: i32,
    pub provider_registration_transaction_id: i32,
    pub masternode_id: i32,
}

// pub fn delete_local_masternodes(chain_id: i32) -> QueryResult<usize> {
//     // need to impl pro_reg_tx and it's join
//     let predicate = schema::local_masternodes::chain_id.eq(chain_id);
//     let source = local_masternodes.filter(predicate);
//     delete(source)
// }


// pub fn local_masternode_for_pro_reg_tx_hash(hash: UInt256) -> QueryResult<LocalMasternode> {
//     let mut pooled_conn = get_pooled_connection();
//     // let predicate  = schema::local_masternodes::provider_registration_transaction
//     local_masternodes::select(local_masternodes, local_masternodes::all_columns())
//         .filter(predicate)
//         .first::<LocalMasternode>(pooled_conn.deref_mut())
// }
