// This script defines a Blockchain structure with methods to manage blocks, transactions, and validate the chain.

use crate::block::Block; // Correct import of Block struct
use crate::transaction::Transaction; // Correct import of Transaction struct

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>, // The chain is a vector of Block structs
    pub current_transactions: Vec<Transaction>, // Current transactions that are yet to be included in a block
}

impl Blockchain {
    // Initializes the Blockchain with a genesis block and returns the new blockchain
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };
        blockchain.create_genesis_block(); // Executes the creation of the genesis block
        blockchain // Returns the initialized blockchain with the genesis block
    }

    // Fills the blockchain with the first block (genesis block)
    fn create_genesis_block(&mut self) {
        // Creates the genesis block with index 0 and an empty previous hash
        let genesis_block = Block::new(0, 0, vec![], "0".to_string());
        self.chain.push(genesis_block); // Adds the genesis block to the blockchain
    }

    // Adds a new block with the current transactions to the blockchain
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block); // Adds the block to the chain
        self.current_transactions.clear(); // Clears the current transactions after adding the block
    }

    // Executes the transaction by creating and adding it to the list of current transactions
    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: u64) {
        let transaction = Transaction::new(sender.to_string(), recipient.to_string(), amount);
        self.current_transactions.push(transaction); // Adds the transaction to the current transaction list
    }

    // Verifies if the blockchain is valid by checking the hash of each block and previous hash consistency
    pub fn is_valid(&mut self) -> bool {
        // Iterates through the blockchain, checking the hash of each block
        for i in 1..self.chain.len() {
            // Borrow the previous block immutably and the current block mutably
            let (previous_block, current_block) = self.chain.split_at_mut(i);
            let current_block = &mut current_block[0]; // This is a mutable reference to the current block
            let previous_block = &previous_block[previous_block.len() - 1]; // This is an immutable reference to the previous block

            // Step 1: Calculate the hash of the current block
            let current_block_hash = current_block.calculate_hash();

            // Step 2: Verify the current block's hash matches its calculated hash
            if current_block.hash != current_block_hash {
                return false; // Returns false if the hashes don't match
            }

            // Step 3: Verify that the previous block's hash matches the current block's previous hash
            if current_block.previous_hash != previous_block.hash {
                return false; // Returns false if previous hashes don't match
            }
        }
        true // Returns true if the blockchain is valid
    }
}
