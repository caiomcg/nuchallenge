use super::{ProviderError, DataProvider};

pub struct StdinProvider {
    input: std::io::Stdin,
}

impl StdinProvider {
    pub fn new() -> Self {
        StdinProvider{
            input: std::io::stdin(),
        }
    }
}

impl DataProvider for StdinProvider {
    fn fetch(&mut self) -> Result<String, ProviderError> {
        let mut content = String::new();

        match self.input.read_line(&mut content) {
            Ok(size) => {
                match size {
                    0 => Err(ProviderError::EOF),
                    _ => Ok(content),
                }
            },
            Err(e) => {
                error!("Could not read from file: {}", e);
                Err(ProviderError::FailedToRead)
            }
        }
    }

    fn consume(&mut self) -> Result<Vec<String>, ProviderError> {
        let mut content = String::new();
        let mut lines = Vec::new();

        while self.input.read_line(&mut content).expect("Could not read line") != 0 {
            lines.push(std::mem::take(&mut content));
        }

        Ok(lines)
    }

    fn reset(&mut self) -> Result<(), ProviderError> {
        Ok(())
    }
}
