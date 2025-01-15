// This script defines a Block structure and methods to create, calculate the hash, and store it for a blockchain system.

use crate::transaction::Transaction;
use serde::{Serialize, Deserialize}; // Imports serde for serialization and deserialization

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u32,           // Defines the block index
    pub timestamp: u64,       // Stores the timestamp of when the block is created
    pub transactions: Vec<Transaction>, // Contains a list of transactions in the block
    pub previous_hash: String, // Stores the hash of the previous block in the chain
    pub hash: String,          // Holds the hash of the current block
}

impl Block {
    // Executes the creation of a new Block with the given index, timestamp, transactions, and previous hash
    pub fn new(index: u32, timestamp: u64, transactions: Vec<Transaction>, previous_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(), // Initializes an empty hash which will be calculated
        };
        block.calculate_hash(); // Executes the hash calculation for the block upon creation
        block // Returns the created block
    }

    // Executes the calculation of the block's hash based on its content
    pub fn calculate_hash(&mut self) -> String {
        let block_data = format!(
            "{}{}{:?}{}", // Formats the block data into a string for hashing
            self.index, self.timestamp, self.transactions, self.previous_hash
        );
        let hash = format!("{:x}", md5::compute(block_data)); // Runs md5 computation on the block data and formats it
        self.hash = hash.clone(); // Stores the computed hash in the block
        hash // Returns the calculated hash
    }
}
