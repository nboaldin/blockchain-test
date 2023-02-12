use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut hash = Sha256::new();
        hash.update(format!("{}{}{}{}", index, timestamp, data, previous_hash).as_bytes());
        let hash = format!("{:x}", hash.finalize());

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
    token_holders: HashMap<String, u64>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            blocks: vec![],
            token_holders: HashMap::new(),
        };
        blockchain.add_block("Genesis Block".to_owned());
        blockchain
    }

    fn add_block(&mut self, data: String) {
        let previous_hash = match self.blocks.last() {
            Some(block) => block.hash.clone(),
            None => "".to_owned(),
        };
        let block = Block::new(self.blocks.len() as u64, data, previous_hash);
        self.blocks.push(block);
    }

    fn issue_tokens(&mut self, holder: String, count: u64) {
        *self.token_holders.entry(holder).or_insert(0) += count;
    }

    fn transfer_tokens(&mut self, from: String, to: String, count: u64) {
        let from_balance = match self.token_holders.get(&from) {
            Some(balance) => *balance,
            None => return,
        };
        if from_balance < count {
            return;
        }
        *self.token_holders.entry(from).or_insert(0) -= count;
        *self.token_holders.entry(to).or_insert(0) += count;
    }

    fn get_token_balance(&self, holder: String) -> u64 {
        *self.token_holders.get(&holder).unwrap_or(&0)
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.issue_tokens("Nando".to_owned(), 100);
    blockchain.add_block("Issued 100 tokend to Nando".to_owned());
    blockchain.issue_tokens("Clarence".to_owned(), 50);
    blockchain.add_block("Issued 50 tokens to Clarence".to_owned());
    blockchain.transfer_tokens("Nando".to_owned(), "Clarence".to_owned(), 25);
    blockchain.add_block("Transferred 25 tokens from Alice to Bob".to_owned());

    println!("Blockchain {:#?}", blockchain);

    println!(
        "Token balance of Alice: {}",
        blockchain.get_token_balance("Alice".to_owned())
    );
    println!(
        "Token balance of Bob: {}",
        blockchain.get_token_balance("Bob".to_owned())
    );
}

