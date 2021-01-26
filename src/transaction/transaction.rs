use chrono::prelude::*;
use super::TransactionError;

#[derive(Debug, Clone)]
pub struct Transaction {
    merchant: String,
    pub amount: i64,
    time: String,
}

impl Transaction {
    pub fn new(merchant: String, amount: i64, time: String) -> Self {
        Transaction {
            merchant,
            amount,
            time,
        }
    }

    pub fn get_time_diff(&self, against: &Transaction) -> Result<chrono::Duration, TransactionError> {
        let lhs_time = self.time.parse::<DateTime<Utc>>();
        let rhs_time = against.time.parse::<DateTime<Utc>>();

        match (&lhs_time, &rhs_time) {
            (Ok(lhs), Ok(rhs)) => Ok(*lhs - *rhs),
            (_, _) => Err(TransactionError::BadTimeFormat),
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.merchant == other.merchant &&
            self.amount == other.amount
    }
}
