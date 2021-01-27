use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    #[serde(rename = "account")]
    Account { 
        #[serde(rename = "available-limit")]
        available_limit: i64,
        #[serde(rename = "active-card")]
        active_card: bool
    },

    #[serde(rename = "transaction")]
    Transaction {
        merchant: String,
        amount: i64,
        time: String,
    }
}

