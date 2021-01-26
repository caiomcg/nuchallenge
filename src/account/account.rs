use itertools::Itertools;

use crate::transaction::Transaction;
use super::ProcessmentError;

#[derive(Debug)]
pub struct Account {
    active_card: bool,
    available_limit: i64,
    violations: Vec<ProcessmentError>,
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(available_limit: i64, active_card: bool) -> Self {
        Account {
            active_card,
            available_limit,
            violations: Vec::new(),
            transactions: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn toggle_active(&mut self) {
        self.active_card = !self.active_card;
    }

    pub fn try_reinitialize(&mut self) {
        self.violations.clear();
        self.violations.push(ProcessmentError::AccountAlreadyInitialized);
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        self.violations.clear();

        match self.active_card {
            true => {
                if let Some(e) = self.contains_duplicate_transaction(&transaction) {
                    self.violations.push(e);
                }

                if let Some(e) = self.processed_multiple_transaction(&transaction) {
                    self.violations.push(e);
                }

                match self.fetch_updated_limit(&transaction) {
                    Ok(limit) => {
                        if self.violations.len() == 0 {
                            self.available_limit = limit;
                        }
                    },
                    Err(e) => self.violations.push(e)
                };

                self.transactions.push(transaction); // perguntar -> Devo inserir uma transacao se a conta não estiver ativa? (ELABORE)
            },
            false => self.violations.push(ProcessmentError::CardNotActive)
        };
    }

    fn fetch_updated_limit(&self, transaction: &Transaction) -> std::result::Result<i64, ProcessmentError> {
        let after_deduction = self.available_limit - transaction.amount;

        match after_deduction {
            _ if after_deduction < 0 => Err(ProcessmentError::InsufficientLimit),
            _ => Ok(after_deduction),
        }
    }

    fn contains_duplicate_transaction(&self, transaction: &Transaction) -> Option<ProcessmentError> {
        for previous_transaction in self.transactions.iter().rev() {
            let divergence = match transaction.get_time_diff(previous_transaction) {
                Ok(diff) => diff,
                Err(e) => {
                    //LOG,
                    return Some(ProcessmentError::InternalServerError);
                }
            }.num_minutes();

            if divergence >= 2 {
                break; // No more transactions to check
            }

            if divergence < 2 && transaction == previous_transaction {
                return Some(ProcessmentError::DoubledTransaction);
            }
        }

        None
    }

    fn processed_multiple_transaction(&mut self, transaction: &Transaction) -> Option<ProcessmentError> {
        if self.transactions.len() > 2 {
            let previous_transaction = self.transactions[self.transactions.len()-2..].first().unwrap();
            let divergence = match transaction.get_time_diff(previous_transaction) {
                Ok(diff) => diff,
                Err(e) => {
                    //LOG,
                    return Some(ProcessmentError::InternalServerError);
                }
            }.num_minutes();

            if divergence < 2 {
                return Some(ProcessmentError::HighFrequencySmallInterval);
            }
        }

        None
    }
}

// TODO: Add a chain of reponsibility to handle transaction errors

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"account": {{"active_card": {}, "available_limit": {}, "violations": [{}]}}}}"#,
               self.active_card,
               self.available_limit,
               self.violations.iter().unique().map(|e| format!("\"{}\"", e)).join(", ")
              )
    }
}
