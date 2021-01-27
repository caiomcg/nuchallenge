use crate::account::ProcessmentError;
use crate::account::validator::{Validator, ValidatorProcessable};

pub struct LimitValidator;

impl LimitValidator {
    pub fn new() -> Self {
        LimitValidator {}
    }
}

impl Validator for LimitValidator {
    fn validate(&self, processable: &mut ValidatorProcessable) {
        let after_deduction = processable.limit - processable.transaction.amount;

        match after_deduction {
            _ if after_deduction < 0 => processable.violations.push(ProcessmentError::InsufficientLimit),
            _ => processable.limit = after_deduction,
        }
    }
}
