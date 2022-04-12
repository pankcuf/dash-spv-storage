use crate::entities::transactions::transaction::Transaction;

#[derive(Debug)]
pub struct Shapeshift<'a> {
    pub error_message: &'a str,
    pub expires_at: i64,
    pub input_address: &'a str,
    pub input_coin_amount: f64,
    pub input_coin_type: &'a str,
    pub is_fixed_amount: bool,
    pub output_coin_amount: f64,
    pub output_coin_type: &'a str,
    pub output_transaction_id: &'a str,
    pub shapeshift_status: i16,
    pub withdrawal_address: &'a str,

    pub transaction: Transaction<'a>,
}
