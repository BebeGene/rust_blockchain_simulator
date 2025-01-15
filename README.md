# Blockchain Simulator

## Description
This project simulates a simple blockchain using Rust. It implements a basic blockchain with features such as block mining (proof of work), transaction processing, and consensus. The blockchain starts with a genesis block and adds new blocks with each mined transaction. The blockchain is initialized with a **genesis block**. You can add transactions and mine new blocks. The blockchain automatically validates its integrity after each new block is added.

## Features
* **Proof of Work (PoW)**: A simple proof of work algorithm for mining blocks.
* **Transactions**: Allows adding transactions to the blockchain.
* **Blockchain Validation**: Verifies the integrity and consistency of the blockchain.

## Prerequisites
* Install [Rust](https://www.rust-lang.org/)
* Clone Repo
* `sha2` crate for hashing

## Run
* cd blockchain_simulator
* cargo run
* When running the script, it will simulate adding transactions and mining a block.

Ex:
Blockchain status: Blockchain { chain: [Block { index: 0, previous_hash: "0", timestamp: 1625232209, transactions: [], nonce: 0, hash: "5e884898da28047151d0e56f8dc6292773603d0d69f3d2198e0c4f914aa60d73" }], current_transactions: [] }

## Screenshots
