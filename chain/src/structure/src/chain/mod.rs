use chrono::Local;
use serde::{Serialize, Deserialize};
use std::{default::Default, mem::size_of_val, collections::HashMap};

use crate::block::Block;

#[derive(Serialize, Deserialize)]
pub struct VilChain {
    pub block_count: usize,
    pub chain_size: usize,
    lookup_table: HashMap<String, Block>,
    chain: Vec<Block>,
    last_block_hash: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for VilChain {
    fn default() -> Self {
        Self {
            block_count: 0,
            chain_size: 0,
            lookup_table: HashMap::new(),
            chain: Vec::with_capacity(1000),
            last_block_hash: None,
            created_at: Local::now().to_string(),
            updated_at: Local::now().to_string(),
        }
    }
}

impl VilChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_last_block_hash(&self) -> Option<String> {
        self.last_block_hash.clone()
    }

    pub fn add_block(&mut self, block: Block) {
        self.block_count += 1;
        self.lookup_table.insert(block.get_block_hash().clone(), block.clone());
        self.chain.push(block);
        self.chain_size = size_of_val(&self);
    } 
}