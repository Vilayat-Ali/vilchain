use serde::{Serialize, Deserialize};
use chrono::Local;
use sha3::{Digest, Sha3_512};

use crate::txn::Txn;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub prev_hash: Option<String>,
    pub hash: String,
    pub txns: Vec<Txn>,
    pub mined_at: String,
    pub mined_by: String, // address of validator
}

impl Block {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockBuilder {
    pub prev_hash: Option<String>,
    pub hash: Option<String>,
    pub txns: Vec<Txn>,
    pub mined_at: Option<String>,
    pub mined_by: Option<String>, // address of validator
}

#[derive(Serialize, Deserialize)]
struct PlaceholderBlock {
    pub prev_hash: Option<String>,
    pub txns: Vec<Txn>,
    pub mined_at: String,
    pub mined_by: String,
}

impl BlockBuilder {
    pub fn new<S>(prev_hash: S) -> Self 
        where S: Into<Option<String>>, 
            {
        Self {
            prev_hash: prev_hash.into(),
            hash: None,
            txns: Vec::with_capacity(5),
            mined_at: None,
            mined_by: None
        }
    }

    pub fn add_txn(&mut self, txn: Txn) -> &mut Self {
        self.txns.push(txn);
        self
    }

    pub fn publish<S>(&mut self, validator_address: S) -> Self 
        where S: Into<String>,
            {
            let placeholder_block = PlaceholderBlock {
                prev_hash: self.prev_hash.clone(),
                txns: self.txns.clone(),
                mined_at: Local::now().to_string(),
                mined_by: validator_address.into(),
            };
            
            


            
            Self {
                prev_hash: self.prev_hash,
                hash
            }
    }
}