use serde::{Serialize, Deserialize};
use chrono::Local;
use sha3::{Sha3_512, Digest};
use std::collections::HashMap;

use crate::txn::Txn;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    prev_hash: Option<String>,
    hash: String,
    merkle_hash: String,
    txns: HashMap<String, Txn>,
    mined_at: String,
    mined_by: String, // address of validator
}

impl Block {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn is_genesis_block(&self) -> bool {
        self.prev_hash.is_none()
    }

    pub fn get_block_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_timestamp_of_mining(&self) -> &String {
        &self.mined_at
    }

    pub fn get_txn_count(&self) -> usize {
        self.txns.len()
    }

    pub fn get_merkle_hash(&self) -> &String {
        &self.merkle_hash
    }

    pub fn get_prev_hash(&self) -> Option<&String> {
        self.prev_hash.as_ref()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockBuilder {
    pub prev_hash: Option<String>,
    pub hash: Option<String>,
    pub merkle_hash: String, // merkle hash of all txns
    pub txns: HashMap<String, Txn>,
    pub mined_at: String,
    pub mined_by: Option<String>, // address of validator
}

#[derive(Serialize, Deserialize)]
struct PlaceholderBlock {
    pub prev_hash: Option<String>,
    pub txns: String, // merkle hash of the txns
    pub mined_at: String, // timestamp when the block was mined
    pub mined_by: String, // node ID or network ID of validator node which mined this particular block of txns
}

impl BlockBuilder {
    pub fn new<S>(prev_hash: S) -> Self 
        where S: Into<Option<String>>, 
            {
        Self {
            prev_hash: prev_hash.into(),
            hash: None,
            txns: HashMap::with_capacity(5),
            mined_at: Local::now().to_string(),
            mined_by: None,
            merkle_hash: String::new(),
        }
    }

    pub fn add_txn(&mut self, txn: Txn) -> &mut Self {
        self.txns.insert(txn.hash.clone(),txn);
        self
    }
    
    fn compute_merkle_hash(txn_list: Vec<Txn>) -> String {
        let combine_hash = |txn_1: &Txn, txn_2: &Txn| {
            let hash_txn_1 = txn_1.hash.clone();
            let concat_hash = hash_txn_1 + &(txn_2.hash);

            let mut hasher = Sha3_512::new();
            hasher.update(serde_json::to_vec(&concat_hash).unwrap());
            
            format!("{:x}", hasher.finalize())
        };

        if txn_list.len() == 0 {
            return String::new();
        }

        if txn_list.len() == 1 {
            return combine_hash(&txn_list[0], &txn_list[0]);
        }

        if txn_list.len() == 2 {
            return combine_hash(&txn_list[0], &txn_list[1]);
        }

        let mid = txn_list.len() / 2;
        let left_hash_vec = txn_list[0..mid].to_vec();
        let right_hash_vec = txn_list[mid..].to_vec();

        let left_hash = BlockBuilder::compute_merkle_hash(left_hash_vec);
        let right_hash = BlockBuilder::compute_merkle_hash(right_hash_vec);

        let mut hasher = Sha3_512::new();
        hasher.update((left_hash + &right_hash).as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn build<S>(&mut self, validator_address: S) -> Block 
        where S: Into<String>,
            {
                let txn_list: Vec<Txn> = self.txns.iter().map(|(_, txn)| {
                        txn.clone()
                    }).collect::<Vec<Txn>>();
                
                let merkle_hash = Self::compute_merkle_hash(txn_list);

                let placeholder_block = PlaceholderBlock {
                    prev_hash: self.prev_hash.clone(),
                    txns: merkle_hash.clone(),
                    mined_at: Local::now().to_string(),
                    mined_by: validator_address.into(),
                };

                let mut hasher = Sha3_512::new();
                hasher.update(serde_json::to_vec(&placeholder_block).unwrap());

                let block_hash = format!("{:x}", hasher.finalize());

            Block {
                prev_hash: placeholder_block.prev_hash.clone(),
                hash: block_hash,
                merkle_hash,
                txns: self.txns.clone(),
                mined_at: placeholder_block.mined_at.clone(),
                mined_by: placeholder_block.mined_by.clone(),
            }
    }
}

#[cfg(test)]
mod tests {
    use crate::txn::UnPublishedTxn;

    use super::*;

    fn generate_random_txns() -> Txn {
        let unpublished_txn = UnPublishedTxn::new("0x12345", "0x67890", "11.2568").unwrap();
        Txn::from(unpublished_txn)
    }

    #[test]
    fn block_creation() {
        let mut builder = BlockBuilder::new(None);

        for _ in 0..5 {
            builder.add_txn(generate_random_txns());
        }

        let mined_block = builder.build("validator_address");

        assert_ne!(mined_block.hash.len(), 0);
        assert_ne!(mined_block.merkle_hash.len(), 0);
    }
}