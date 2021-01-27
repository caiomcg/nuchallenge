use crate::account::ProcessmentError;
use crate::account::validator::{Validator, ValidatorProcessable};

pub struct MultipleTransactionValidator;

impl MultipleTransactionValidator {
    pub fn new() -> Self {
        MultipleTransactionValidator{}
    }
}

impl Validator for MultipleTransactionValidator {
    fn validate(&self, processable: &mut ValidatorProcessable) {
        let transactions = processable.account.get_transaction_history();
        if transactions.len() > 2 {
            let previous_transaction = transactions[transactions.len()-2..].first().unwrap();
            let divergence = match processable.transaction.get_time_diff(previous_transaction) {
                Ok(diff) => diff,
                Err(e) => {
                    error!("Could not calculate time divergency: {}", e);
                    processable.violations.push(ProcessmentError::InternalServerError);
                    return;
                }
            }.num_minutes();

            if divergence < 2 {
                processable.violations.push(ProcessmentError::HighFrequencySmallInterval);
            }
        }
    }
}
