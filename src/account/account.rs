use itertools::Itertools;

use super::ProcessmentError;

use crate::transaction::Transaction;
use crate::mapping::{Message, MappingError};
use crate::account::validator::{Validator, ValidatorProcessable};


pub struct Account {
    active_card: bool,
    available_limit: i64,
    violations: Vec<ProcessmentError>,
    transactions: Vec<Transaction>,
    validators: Vec<Box<dyn Validator>>,
}

impl Account {
    pub fn new(available_limit: i64, active_card: bool) -> Self {
        let account = Account {
            active_card,
            available_limit,
            violations: Vec::new(),
            transactions: Vec::new(),
            validators: Vec::new(),
        };

        debug!("Registered new account");

        account
    }

    pub fn get_available_limit(&self) -> i64 {
        self.available_limit
    }

    pub fn get_transaction_history(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    #[allow(dead_code)]
    pub fn toggle_active(&mut self) {
        debug!("Toogled account active_card to {}", self.active_card);
        self.active_card = !self.active_card;
    }

    pub fn try_reinitialize(&mut self) {
        error!("Trying to recreate an account");
        self.violations.clear();
        self.violations.push(ProcessmentError::AccountAlreadyInitialized);
    }

    pub fn register_validator(&mut self, validator: Box<dyn Validator>) -> &mut Self {
        self.validators.push(validator);
        self
    }

    pub fn process_transaction(&mut self, transaction: Transaction) {
        debug!("Processing a new transaction {:?}", transaction);

        self.violations.clear();

        match self.active_card {
            true => {
                let mut processable = ValidatorProcessable::new(self, &transaction);

                for validator in &self.validators {
                    validator.validate(&mut processable);
                }

                match processable.violations.len() {
                    0 => self.available_limit = processable.limit,
                    _ => self.violations = processable.violations,
                }

                self.transactions.push(transaction);
            },
            false => self.violations.push(ProcessmentError::CardNotActive)
        };
    }

}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"account": {{"active_card": {}, "available_limit": {}, "violations": [{}]}}}}"#,
               self.active_card,
               self.available_limit,
               self.violations.iter().unique().map(|e| format!("\"{}\"", e)).join(", ")
              )
    }
}

impl std::convert::TryFrom<Message> for Account {
    type Error = MappingError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Account { available_limit, active_card }=> Ok(Account::new(available_limit, active_card)),
            Message::Transaction { .. } => Err(MappingError::AccountFromTransaction),
        }
    }
}
