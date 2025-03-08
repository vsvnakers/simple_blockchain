use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}
