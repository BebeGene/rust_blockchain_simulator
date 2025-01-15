// This script defines a Transaction struct and provides a method to create new transactions.

use serde::{Serialize, Deserialize}; // Imports serde for serialization and deserialization

#[derive(Serialize, Deserialize, Debug, Clone)] // Derives serialization, deserialization, debug, and cloning capabilities for the Transaction struct
pub struct Transaction {
    pub sender: String,   // Defines the sender of the transaction
    pub recipient: String, // Defines the recipient of the transaction
    pub amount: u64,       // Defines the amount being transferred
}

impl Transaction {
    // Executes the creation of a new transaction with the given sender, recipient, and amount
    pub fn new(sender: String, recipient: String, amount: u64) -> Transaction {
        Transaction { sender, recipient, amount } // Returns a new Transaction instance
    }
}
