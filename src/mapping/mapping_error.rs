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

mod test {
    use super::MappingError;

    #[test]
    fn format_accout_from_transition_error() {
        let error = MappingError::AccountFromTransaction;
        let formatted = format!("{}", error);

        assert_eq!(formatted, "account-from-transaction");
    }

    #[test]
    fn format_transaction_from_account() {
        let error = MappingError::TransactionFromAccount;
        let formatted = format!("{}", error);

        assert_eq!(formatted, "transaction-from-account");
    }
}
