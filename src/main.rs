#[macro_use]
extern crate log;

mod transaction;
mod account;
mod data_provider;
mod mapping;

use std::convert::TryFrom;

use transaction::Transaction;
use account::Account;
use account::ProcessmentError;
use data_provider::provider_factory::{ProviderType, ProviderFactory};
use mapping::Message;
use account::validator::{
    LimitValidator,
    DoubleTrasactionValidator,
    MultipleTransactionValidator,
};


fn add_validators(to: &mut Box<Account>) {
    to.register_validator(Box::new(LimitValidator::new()))
        .register_validator(Box::new(DoubleTrasactionValidator::new()))
        .register_validator(Box::new(MultipleTransactionValidator::new()));

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut account: Option<Box<Account>> = None;
    let mut data_provider = match std::env::var("AUTHORIZER_FILE") {
        Ok(file) => ProviderFactory::create_provider(ProviderType::File(&file)),
        Err(_) => {
            info!("Environment variable not found, dropping to stdin");
            ProviderFactory::create_provider(ProviderType::Stdin)
        }
    };

    while let Ok(string) = data_provider.fetch() {
        let message: Message = serde_json::from_str(&string)?;

        match message {
            message @ Message::Account { .. } => {
                if let Some(ref mut acc) = account {
                    acc.try_reinitialize();
                    println!("{}", acc);
                } else {
                    let mut boxed_account = Box::new(Account::try_from(message)?);
                    add_validators(&mut boxed_account);
                    account = Some(boxed_account);
                    println!("{}", account.as_ref().unwrap());
                }
            },
            message @ Message::Transaction { .. } => {
                if let Some(ref mut acc) = account {
                    acc.process_transaction(Transaction::try_from(message)?);
                    println!("{}", acc);
                } else {
                    println!(r#"{{"violations": ["{}"]}}"#, ProcessmentError::AccountNotInitialized.to_string()); // perguntar -> Como formatar?
                }
            }
        };
    }
    Ok(())
}
