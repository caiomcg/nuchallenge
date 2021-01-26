#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ProcessmentError {
    InsufficientLimit,
    HighFrequencySmallInterval,
    AccountAlreadyInitialized,
    AccountNotInitialized,
    DoubledTransaction,
    CardNotActive,
    InternalServerError,
}

impl std::fmt::Display for ProcessmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessmentError::InsufficientLimit => write!(f, "insufficient-limit"),
            ProcessmentError::HighFrequencySmallInterval => write!(f, "high-frequency-small-interval"),
            ProcessmentError::AccountAlreadyInitialized => write!(f, "account-already-initialized"),
            ProcessmentError::AccountNotInitialized => write!(f, "account-not-initialized"),
            ProcessmentError::DoubledTransaction => write!(f, "doubled-transaction"),
            ProcessmentError::CardNotActive => write!(f, "card-not-active"),
            ProcessmentError::InternalServerError => write!(f, "internal-server-error"),
        }
    }
}
