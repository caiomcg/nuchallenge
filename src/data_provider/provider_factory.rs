use super::{FileProvider, StdinProvider, DataProvider};

pub enum ProviderType<'a> {
    File(&'a str),
    Stdin,
}

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create_provider(with_type: ProviderType) -> Box<dyn DataProvider> {
        match with_type {
            ProviderType::File(for_path) => Box::new(FileProvider::new(for_path)),
            ProviderType::Stdin => Box::new(StdinProvider::new()),
        }
    }
}
