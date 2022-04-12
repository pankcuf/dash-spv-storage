use crate::entities::transactions::special_transaction::SpecialTransaction;

#[derive(Debug)]
pub struct CoinbaseTransaction<'a> {
    pub base: SpecialTransaction<'a>,

    pub height: i32,
    pub merkle_root_mn_list: &'a [u8],
}
