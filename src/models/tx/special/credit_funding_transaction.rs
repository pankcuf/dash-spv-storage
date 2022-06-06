use crate::schema::credit_funding_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct CreditFundingTransaction {
    pub id: i32,
    pub base_id: i32,
    pub registered_blockchain_identity_id: i32,
    pub topped_up_blockchain_identity_id: i32,

}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="credit_funding_transactions"]
pub struct NewCreditFundingTransaction {
    pub base_id: i32,
    pub registered_blockchain_identity_id: i32,
    pub topped_up_blockchain_identity_id: i32,
}
