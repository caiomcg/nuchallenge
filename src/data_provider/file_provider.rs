use super::{ProviderError, DataProvider};
use std::io::{Seek, BufRead, Read};

pub struct FileProvider {
    buffer: std::io::BufReader<std::fs::File>,
}

impl FileProvider {
    pub fn new(for_path: &str) -> Self {
        let file = std::fs::File::open(for_path).expect("Could not open the desided file");

        FileProvider {
            buffer: std::io::BufReader::new(file),
        }
    }
}

impl DataProvider for FileProvider {
    fn fetch(&mut self) -> Result<String, ProviderError> {
        let mut content = String::new();

        match self.buffer.read_line(&mut content) {
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
        Ok(self.buffer.by_ref().lines()
           .map(|l| l.expect("Could not parse line"))
           .collect())
    }

    fn reset(&mut self) -> Result<(), ProviderError> {
        match self.buffer.seek(std::io::SeekFrom::Start(0)) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Could not seek on the input {}", e);
                Err(ProviderError::CouldNotReset)
            },
        }
    }
}
