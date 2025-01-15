// This script initializes a blockchain, adds transactions, creates new blocks, and validates the blockchain.

mod blockchain;
mod block;
mod transaction;

use blockchain::Blockchain;    // Imports the Blockchain struct
use block::Block;              // Imports the Block struct
// use transaction::Transaction;  // Commented out import of Transaction struct

fn main() {
    // Initializes the blockchain with a genesis block
    let mut blockchain = Blockchain::new();

    // Adds transactions to the blockchain
    blockchain.new_transaction("Alice", "Bob", 50); // Executes the creation of a new transaction
    blockchain.new_transaction("Bob", "Charlie", 30); // Executes the creation of another new transaction

    // Creates a new block using the current transactions
    let new_block = Block::new(
        blockchain.chain.len() as u32, // Sets the block's index based on the current chain length
        1234567890,                    // Sets the timestamp for the block
        blockchain.current_transactions.clone(), // Passes the current transactions to the new block
        blockchain.chain.last().unwrap().hash.clone(), // Sets the previous block's hash
    );
    blockchain.add_block(new_block); // Adds the newly created block to the blockchain

    // Prints all blocks in the blockchain
    println!("Blockchain contains:");
    for block in &blockchain.chain {
        println!("{:?}", block); // Prints each block's details
    }

    // Validates the blockchain's integrity
    if blockchain.is_valid() { 
        println!("Blockchain is valid."); // Prints if the blockchain is valid
    } else {
        println!("Blockchain is invalid."); // Prints if the blockchain is invalid
    }

    // Adds another transaction to the blockchain
    blockchain.new_transaction("Charlie", "David", 20); // Executes the creation of a new transaction

    // Creates another block with the new transactions
    let another_block = Block::new(
        blockchain.chain.len() as u32, // Sets the block's index based on the current chain length
        9876543210,                    // Sets the timestamp for the block
        blockchain.current_transactions.clone(), // Passes the current transactions to the new block
        blockchain.chain.last().unwrap().hash.clone(), // Sets the previous block's hash
    );
    blockchain.add_block(another_block); // Adds the newly created block to the blockchain

    // Validates the blockchain again after adding the new block
    if blockchain.is_valid() {
        println!("Blockchain is valid after adding another block."); // Prints if the blockchain is valid after the new block
    } else {
        println!("Blockchain is invalid after adding another block."); // Prints if the blockchain is invalid after the new block
    }
}
