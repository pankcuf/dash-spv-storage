use crate::entities::derivation_path::DerivationPath;
use crate::entities::masternode::masternode_entry::MasternodeEntry;
use crate::entities::transactions::special_transaction::SpecialTransaction;
use crate::entities::transactions::transaction_input::TransactionInput;
use crate::entities::transactions::transaction_output::TransactionOutput;

#[derive(Debug)]
pub struct Address<'a> {
    pub address: &'a str,
    pub identity_index: i32,
    pub index: i32,
    pub internal: bool,
    pub standalone: bool,

    pub derivation_path: DerivationPath<'a>,
    pub used_in_inputs: Vec<TransactionInput<'a>>,
    pub used_in_outputs: Vec<TransactionOutput<'a>>,
    pub used_in_masternode_entries: Vec<MasternodeEntry<'a>>,
    pub used_in_special_transactions: Vec<SpecialTransaction<'a>>,
}
