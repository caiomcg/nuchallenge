use super::ProviderError;

pub trait DataProvider {
    fn fetch(&mut self) -> Result<String, ProviderError>;
    fn consume(&mut self) -> Result<Vec<String>, ProviderError>;
    fn reset(&mut self) -> Result<(), ProviderError>;
}
