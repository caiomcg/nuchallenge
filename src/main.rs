mod transaction;
mod account;

use serde_json::Value;

use transaction::Transaction;
use account::Account;
use account::ProcessmentError;

fn main() {
    let mut account: Option<Box<Account>> = None;

    let sample_strings = vec![
r#"{"account": {"active-card": true, "available-limit": 100000000}}"#,
r#"{"transaction": {"merchant": "Burger King 44", "amount": 20, "time":"2019-02-13T10:08:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 44", "amount": 20, "time":"2019-02-13T10:08:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 45", "amount": 20, "time":"2019-02-13T10:08:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 46", "amount": 20, "time":"2019-02-13T10:08:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 44", "amount": 20, "time":"2019-02-13T10:08:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:01.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 50", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 51", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 52", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 52", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 52", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 52", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 53", "amount": 70, "time":"2019-02-13T10:20:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 1, "time":"2019-02-13T10:20:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 50", "amount": 9, "time":"2019-02-13T10:20:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 50", "amount": 9, "time":"2019-02-13T10:20:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 49", "amount": 20, "time":"2019-02-13T10:10:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 60", "amount": 20, "time":"2019-02-13T10:20:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 61", "amount": 20, "time":"2019-02-13T10:25:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 62", "amount": 20, "time":"2019-02-13T10:30:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 63", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 64", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 20, "time":"2019-02-13T10:35:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 15, "time":"2019-02-13T10:40:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 18, "time":"2019-02-13T10:40:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 117, "time":"2019-02-13T10:40:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 100000, "time":"2019-02-13T10:45:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 550, "time":"2019-02-13T10:45:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 89899550, "time":"2019-02-13T10:45:05.000Z"}}"#,
r#"{"transaction": {"merchant": "Burger King 65", "amount": 89899550, "time":"2019-02-13T10:45:05.000Z"}}"#
    ];
    for string in sample_strings {
        //println!("\nIn: {}", string);
        match serde_json::from_str::<Value>(string) {
            Ok(json) => {
                if let Some(content) = json.get("account") {
                    match (content["available-limit"].as_i64(), content["active-card"].as_bool()) {
                        (Some(limit), Some(active)) => {
                            if let Some(ref mut acc) = account {
                                acc.try_reinitialize();
                                println!("{}", acc);
                            } else {
                                account = Some(Box::new(Account::new(limit, active)));
                                println!("{}", account.as_ref().unwrap());
                            }
                        },
                        (_, _) => println!("Cannot process the input with incomplete data"),
                    };
                } 

                if let Some(content) = json.get("transaction") {
                    match (content["merchant"].as_str(), content["amount"].as_i64(), content["time"].as_str()) {
                        (Some(merchant), Some(amount), Some(time)) => {
                            if let Some(ref mut acc) = account {
                                acc.process_transaction(Transaction::new(merchant.to_string(), amount, time.to_string()));
                                println!("{}", acc);
                            } else {
                                println!(r#"{{"violations": ["{}"]}}"#, ProcessmentError::AccountNotInitialized.to_string()); // perguntar -> Como formatar?
                            }
                        }
                        (_, _, _) => println!("Cannot process the input with incomplete data"),
                    };
                }
            },
            Err(e) => println!("Error: {}", e),
        };
    }
}
