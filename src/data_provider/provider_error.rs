#[derive(Debug)]
pub enum ProviderError {
    EOF,
    CouldNotReset,
    UnknownFile,
    FailedToRead,
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::EOF => write!(f, "eof"),
            ProviderError::CouldNotReset => write!(f, "could-not-reset"),
            ProviderError::UnknownFile => write!(f, "unknown-file"),
            ProviderError::FailedToRead => write!(f, "failed-to-read"),
        }
    }
}

impl std::error::Error for ProviderError {}
