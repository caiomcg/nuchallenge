use crate::account::ProcessmentError;
use crate::account::validator::{Validator, ValidatorProcessable};

pub struct DoubleTrasactionValidator;

impl DoubleTrasactionValidator {
    pub fn new() -> Self {
        DoubleTrasactionValidator{}
    }
}

impl Validator for DoubleTrasactionValidator {
    fn validate(&self, processable: &mut ValidatorProcessable) {
        for previous_transaction in processable.account.get_transaction_history().iter().rev() {
            let divergence = match processable.transaction.get_time_diff(previous_transaction) {
                Ok(diff) => diff,
                Err(e) => {
                    error!("Could not calculate time divergency: {}", e);
                    processable.violations.push(ProcessmentError::InternalServerError);
                    return;
                }
            }.num_minutes();

            if divergence >= 2 {
                break; // No more transactions to check
            }

            if divergence < 2 && processable.transaction == previous_transaction {
                processable.violations.push(ProcessmentError::DoubledTransaction);
            }
        }
    }
}
