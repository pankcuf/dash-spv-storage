use crate::entities::account::Account;
use crate::entities::dashpay_user::DashpayUser;
use crate::entities::derivation_path::DerivationPath;

#[derive(Debug)]
pub struct FriendRequest<'a> {
    pub destination_key_index: i32,
    pub friendship_id: &'a [u8],
    pub source_key_index: i32,
    pub timestamp: i64,

    pub account: Account<'a>,
    pub derivation_path: DerivationPath<'a>,
    pub destination_contact: DashpayUser<'a>,
    pub source_contact: DashpayUser<'a>,
}
