use chrono::NaiveDateTime;
use crate::schema::shapeshifts;
/// queries:
/// "(shapeshiftStatus == %@) || ((shapeshiftStatus == %@) && (SUBQUERY(transaction.outputs, $output, ($output.shapeshiftOutboundAddress != NIL)).@count == 1))"
/// indexation:
/// "expiresAt": DESC
#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Shapeshift {
    pub id: i32,
    pub transaction_id: i32,

    pub input_coin_amount: f64,
    pub output_coin_amount: f64,

    pub expires_at: NaiveDateTime,

    pub shapeshift_status: i16,
    pub is_fixed_amount: bool,

    pub error_message: String,
    pub input_address: String,
    pub input_coin_type: String,
    pub output_coin_type: String,
    pub output_transaction_id: String,
    pub withdrawal_address: String,

}

#[derive(Insertable, PartialEq, Debug)]
#[table_name="shapeshifts"]
pub struct NewShapeshift<'a> {
    pub transaction_id: i32,

    pub input_coin_amount: f64,
    pub output_coin_amount: f64,

    pub expires_at: NaiveDateTime,

    pub shapeshift_status: i16,
    pub is_fixed_amount: bool,

    pub error_message: &'a str,
    pub input_address: &'a str,
    pub input_coin_type: &'a str,
    pub output_coin_type: &'a str,
    pub output_transaction_id: &'a str,
    pub withdrawal_address: &'a str,
}
