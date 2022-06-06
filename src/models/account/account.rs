use std::ops::DerefMut;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryResult, RunQueryDsl, Table};
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::query_dsl::methods::SelectDsl;
use crate::{create, get_pooled_connection, schema};
use crate::schema::accounts;

#[derive(Identifiable, Queryable, PartialEq, Eq, Clone, Debug)]
pub struct Account {
    pub id: i32,
    pub index: i32,
    pub chain_id: i32,
    pub wallet_unique_id: String,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub index: i32,
    pub chain_id: i32,
    pub wallet_unique_id: &'a str,
}

pub fn create_account<'a>(wallet_unique_id: &'a str, index: i32, chain_id: i32) -> QueryResult<usize> {
    let records = NewAccount {
        index,
        chain_id,
        wallet_unique_id
    };
    let target = accounts::dsl::accounts;
    create(target, &records)
}

pub fn account_for_wallet_unique_id(wallet_unique_id: String, index: i32, chain_id: i32) -> Account {
    let mut pooled_conn = get_pooled_connection();
    let predicate = accounts::wallet_unique_id.eq(wallet_unique_id.clone())
        .and(accounts::index.eq(index));
    match schema::accounts::dsl::accounts::select(accounts::dsl::accounts, accounts::dsl::accounts::all_columns())
        .filter(predicate)
        .get_results::<Account>(pooled_conn.deref_mut()) {
        Ok(accounts) => {
            assert_eq!(accounts.len(), 1, "There can only be one account per index per wallet");
            return accounts[0].clone();
        },
        Err(err) => {
            println!("{}", err.to_string());
            match create_account(wallet_unique_id.clone().as_str(), index, chain_id) {
                Ok(_size) => account_for_wallet_unique_id(wallet_unique_id, index, chain_id),
                Err(err) => {
                    println!("{}", err.to_string());
                    panic!("Error saving account {}", err.to_string());
                }
            }
        }
    }
}
