use crate::entities::instant_send_lock::InstantSendLock;
use crate::entities::shapeshift::Shapeshift;
use crate::entities::transactions::transaction_hash::TransactionHash;
use crate::entities::transactions::transaction_input::TransactionInput;
use crate::entities::transactions::transaction_output::TransactionOutput;

#[derive(Debug)]
pub struct Transaction<'a> {
    pub lock_time: i32,

    pub associated_shapeshift: Shapeshift<'a>,
    pub inputs: Vec<TransactionInput<'a>>,
    pub instant_send_lock: InstantSendLock<'a>,
    pub outputs: Vec<TransactionOutput<'a>>,
    pub transaction_hash: TransactionHash<'a>,
}
