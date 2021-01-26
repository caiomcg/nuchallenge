#[derive(Eq, PartialEq, Hash, Debug)]
pub enum TransactionError {
    BadTimeFormat,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::BadTimeFormat => write!(f, "bad-time-format"),
        }
    }
}
