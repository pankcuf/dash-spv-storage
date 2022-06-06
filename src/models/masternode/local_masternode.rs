use crate::schema::local_masternodes;

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
