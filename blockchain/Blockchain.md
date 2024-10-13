# Simple Blockchain in Rust
--------------------------------------------------------
## Blockchain Theory

- #### Blockchain technology, helps in securing transaction, reduce compliance cost and speed up data transfer.
- These objectives are to great extent aligned with the Rust objectives, that is, **Security** and **Speed**. This makes rust one of the perfect candidates for deploying and developing blockchain as the underlying environment before jumping into the implementation details.
--------------------------------------------------------
### Part 1: Hashes
========================================================
- Consider a scenario in which you have a very large file. Let's say 1 GB of file and you want to send it over to a friend across the globe. Now you don't trust the carrier of the file and you want to make sure that the file is not tampered with or corrupted during the transit. You are considered about even one bit changing. 
- You also don't want to send it over a secure channel, because that will be slow and expensive. So you want to send it over a normal channel. what do you do?
- You can take the file, read it as bytes, and put it in a function which is going to compute a much smaller representation of the file. This smaller representation is called a **hash**. Usually, currently (circa 2024) it is done using **Secure Hash Algorithm (SHA)**, which applies several mathematical operations to compute hash of the given data.
- In simple words, a **hash** can be treated as fingerprints of the data. If the data changes, the hash will change. If the data is same, the hash will be same.
- **NOTE that hash, if calculated using same algorithm, will be of same size for any data. For example, SHA256 will always produce 256 bits hash, whether your data is composed of one character or multiple GBs.**
- **NOTE that hash is a one-way function, meaning you can't get the original data back from the hash. Meaning if you only have a hash, you can't get the original data back.**
- **NOTE that hash is deterministic, meaning if you have the same data, you will always get the same hash if you calculate it using the same algorithm.**
- **NOTE that hash is unique, meaning if you have different data, even by a single bit, you will always get different hash.**
- The hashes are very old concept and have many use cases.
    - If you want to store some big file on some insecure data storage, but you don't need the same file right now. But in future you may need it and you don't know when. So you can store the hash of the file and when you need the file, you can calculate the hash of the file and compare it with the stored hash. If they match, you can be sure that the file is not tampered with or corrupted.

========================================================
### Part 2: Blocks and Blockchains
========================================================
- A block will have some additional fields in addition for data for hashing.
- **First Field** is called **block**. Usually some kind of number.
- **Second Field** is called **Nonce**. Which again is usually a number.
- **Third Field** is the data that you want to store in the block.
- Hash is the hash of the all three fields.
- Hash in this case will start with four zeros, `0000`.
- Most of the hash will not generally start with like this. So you have to keep changing the nonce until you get the hash that starts with four zeros. When you get to that hash, you can say that you have mined the block, and block is said to be signed.
- #### To make sure that block will not be tempered we will put some restrictions.
    - A block is a valid block if the hash of all of its fields come out to some hash that starts with four zeros. The chances of hash being `0000` is quite low, near to 1 in 2^16. So you have to keep changing the nonce until you get the hash that starts with four zeros.
- To get a hash with four zeros, we can adjust `Nonce`. Let's see how we will do this.
- One of the way to adjust the nonce is to keep incrementing it until we get the hash that starts with four zeros.
- #### `Mining` - The process of finding the nonce that will give us the hash that starts with four zeros is called mining. This makes the block valid and signed.
- #### `Blockchain` - A chain of blocks, where each block is signed and valid, is called a blockchain.
- For a blockchain, we will have a genesis block, which is the first block in the chain. This block will not have any previous block hash, because it is the first block. So we will put some default value in the previous block hash field.
- The genesis block will have a block number of 0 and nonce of 0. The data can be anything.
- The hash of the genesis block will be calculated by hashing the block number, nonce, and data. The hash will start with four zeros.
- The next block will have the previous block hash as the hash of the genesis block. The block number will be 1 and nonce will be **mined**. The data can be anything.
- What will happen if we change some data in some intermediate block? Let's say the block just before the last block. Let's consider you have 5 block in the blockchain. If someone changes some part of data in 4th block, its hash will change, and either it will not be a valid block, so block 5, which takes hash of block 4 as previous block hash, will not be valid. So the whole blockchain will be invalid. So both block 4 and block 5 will become invalid.
- In the real world, the hash requirement has far more `0`s than just 4. It can be 20, 30, or even 60. The more the number of zeros, the more difficult it is to mine the block. This is called `difficulty`. The difficulty is adjusted by the network to make sure that the block is mined in a certain time. This is called `Proof of Work`. The more the number of zeros, the more difficult it is to mine the block. So by the time, you figure out a valid nonce for some older block, chain will have some more blocks. So you have to keep up with the network to mine the block.
- Blockchain usually have a distributed nature, with each distribution point, having full blockchain. This is called `decentralization`. This is done to make sure that no single entity can control the blockchain. This is called `51% attack`. If some entity has 51% of the network, it can control the blockchain. So to make sure that no single entity can control the blockchain, it is distributed.
- **Decentralization** Also makes sure that if some individual blockchain is tempered, then by democratic voting, the network can decide which blockchain is valid. This is called `consensus`.
- These things make sure that blockchain is secure, tamper-proof, and decentralized. Tampering with blockchain is made very very difficult, and it is not worth it. This is why blockchain is considered secure.
- Usually blockchain is used for financial transactions, but it can be used for any kind of data that you want to store securely and tamper-proof. Financial transaction or smart contracts are put in the data field of the block.
- **NOTE that blockchain is not a database. It is a ledger. It is a chain of blocks. It is not a database.**

--------------------------------------------------------
## Implementation of Blockchain in Rust
--------------------------------------------------------
```rust
use chrono::Utc;
use sha3::{Digest, Sha3_256};

/// Blockchain -> Basically a vector of blocks
#[derive(Debug, Clone)]
struct BlockChain {
    blocks: Vec<Block>,
}

/// Block -> Contains the id (block number), data, hash, previous hash, timestamp, and nonce
#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl BlockChain {
    /// Create a new blockchain, by initializing the blocks vector
    fn new() -> Self {
        Self {
            blocks: vec![], // Empty vector
        }
    }

    fn generate_genesis_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            data: "I am the first or genesis block".to_string(),
            previous_hash: [0; 64]
                .into_iter()
                .map(|i| i.to_string())
                .collect::<String>(),
            nonce: 11316,
            hash: String::from("000015783b764259d382017d91a36d206d0600e2cbb3567748f46a33fe9297cf"),
            timestamp: Utc::now().timestamp(),
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            None => {
                println!(
                    "The block chain has no genesis block. Please generate a genesis block first."
                );
                return;
            }
            Some(last_block) => {
                if self.is_block_valid(&block, last_block) {
                    self.blocks.push(block);
                    println!("Block added to the blockchain.");
                } else {
                    println!("Block is not valid. Block not added to the blockchain.");
                }
            }
        }
    }

    fn is_block_valid(&self, new_block: &Block, last_block: &Block) -> bool {
        // Check if new blocks previous hash is equal to the last block's hash
        if new_block.previous_hash != last_block.hash {
            println!("Invalid block. Previous hash does not match last block's hash.");
            return false;
        }

        if !new_block.hash.starts_with("0000") {
            println!("Invalid block. Hash does not start with 0000");
            return false;
        }

        if new_block.id != last_block.id + 1 {
            println!("Invalid block. Block id is not last block's id + 1");
            return false;
        }

        let mut hasher = Sha3_256::new();
        let block_string = format!(
            "{}{}{}{}{}",
            new_block.id,
            new_block.previous_hash,
            new_block.data,
            new_block.timestamp,
            new_block.nonce
        );
        hasher.update(block_string);
        let hash = hasher.finalize();
        let hash = hash
            .iter()
            .map(|i| format!("{:02x}", i))
            .collect::<String>();
        if hash != new_block.hash {
            println!("Invalid block. Hash does not match the hash of the block.");
            return false;
        }
        true
    }

    fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        match chain.len() {
            0 => println!("The chain is empty."),
            1 => println!("The chain has only one block."),
            _ => {
                for i in 1..chain.len() {
                    let previous = chain.get(i - 1).unwrap();
                    let current = chain.get(i).unwrap();
                    if !self.is_block_valid(current, previous) {
                        return false;
                    }
                }
            }
        }

        println!("The chain is found to be correct and valid.");
        true
    }

    /// Update the chain if there is update on other decentralized nodes
    fn chain_selector(&self, local: Vec<Block>, remote: Vec<Block>) -> Option<Vec<Block>> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        match (is_local_valid, is_remote_valid) {
            (true, true) => {
                if local.len() >= remote.len() {
                    println!("The local copy is valid and more current. Returning local copy.");
                    Some(local)
                } else {
                    println!("The remote copy is valid and more current. Returning remote copy.");
                    Some(remote)
                }
            }
            (true, false) => {
                println!(
                    "The local copy is valid but remote copy is invalid. Returning local copy."
                );
                Some(local)
            }
            (false, true) => {
                println!(
                    "The remote copy is valid but local copy is invalid. Returning remote copy."
                );
                Some(remote)
            }
            (false, false) => {
                println!("Both local and remote copies are invalid. Returning None.");
                None
            }
        }
    }
}

impl Block {
    /// Create a new block
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let now = Utc::now().timestamp();

        let (nonce, hash) = Block::mine_block(id, now, &previous_hash, &data);

        Self {
            id,
            data,
            previous_hash,
            nonce,
            hash,
            timestamp: now,
        }
    }

    /// Mine a new block, outputs a tuple of Nonce (valid nonce) and Hash (hash of the block)
    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
        println!("Mining block ....");
        let mut nonce = 0 as u64;
        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);
            let mut hasher = Sha3_256::new();
            hasher.update(block_string);
            let hash = hasher.finalize();
            let hash = hash
                .iter()
                .map(|i| format!("{:02x}", i))
                .collect::<String>();
            if hash.starts_with("0000") {
                println!("Mining Completed!.\nNonce: {}\nHash: {}", nonce, hash);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let mut new_blockchain = BlockChain::new();
    new_blockchain.generate_genesis_block();

    println!("{:?}", new_blockchain);

    let last_block = new_blockchain.blocks.last().unwrap();
    let new_block = Block::new(
        last_block.id + 1,
        last_block.hash.to_owned(),
        "Shubhendu A. Dutta".to_string(),
    );

    new_blockchain.try_add_block(new_block);

    new_blockchain.is_chain_valid(&new_blockchain.blocks);

    println!();
    let last_block = new_blockchain.blocks.last().unwrap();
    let new_block = Block::new(
        last_block.id + 1,
        last_block.hash.to_owned(),
        "I currently work at Deloitte Digital".to_string(),
    );
    new_blockchain.try_add_block(new_block);

    println!();
    let last_block = new_blockchain.blocks.last().unwrap();
    let new_block = Block::new(
        last_block.id + 1,
        last_block.hash.to_owned(),
        "I am a Blockchain Developer".to_string(),
    );
    new_blockchain.try_add_block(new_block);

    println!();
    println!("Validating the chain ....");
    new_blockchain.is_chain_valid(&new_blockchain.blocks);

    // Chain selector
    println!();
    new_blockchain.blocks = new_blockchain
        .chain_selector(
            new_blockchain.blocks.to_owned(),
            new_blockchain.blocks.to_owned(),
        )
        .unwrap();
}
```