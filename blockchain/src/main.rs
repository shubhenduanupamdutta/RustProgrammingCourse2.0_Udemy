//----------------------------------------------------------------
//       Blockchain in Rust from scratch
//----------------------------------------------------------------
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
