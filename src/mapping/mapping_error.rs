#[derive(Debug)]
pub enum MappingError {
    AccountFromTransaction,
    TransactionFromAccount,
}

impl std::fmt::Display for MappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingError::AccountFromTransaction => write!(f, "account-from-transaction"),
            MappingError::TransactionFromAccount => write!(f, "transaction-from-account"),
        }
    }
}

impl std::error::Error for MappingError {}
