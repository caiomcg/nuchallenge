use crate::account::Account;
use crate::account::ProcessmentError;
use crate::transaction::Transaction;

pub struct ValidatorProcessable<'a> {
    pub account: &'a Account,
    pub transaction: &'a Transaction,
    pub limit: i64,
    pub violations: Vec<ProcessmentError>,
}

impl<'a> ValidatorProcessable<'a> {
    pub fn new(account: &'a Account, transaction: &'a Transaction) -> Self {
        ValidatorProcessable {
            account,
            transaction,
            limit: account.get_available_limit(),
            violations: Vec::new(),
        }
    }
}

pub trait Validator {
    fn validate(&self, processable: &mut ValidatorProcessable);
}
