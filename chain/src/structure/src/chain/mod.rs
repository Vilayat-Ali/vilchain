use chrono::Local;
use serde::{Serialize, Deserialize};
use std::{default::Default, mem::size_of_val, collections::HashMap};

use crate::block::Block;

#[derive(Serialize, Deserialize)]
pub struct VilChain {
    pub block_count: usize,
    pub chain_size: usize,
    blocks: HashMap<String, Block>,
    last_block_hash: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for VilChain {
    fn default() -> Self {
        Self {
            block_count: 0,
            chain_size: 0,
            blocks: HashMap::new(),
            last_block_hash: None,
            created_at: Local::now().to_string(),
            updated_at: Local::now().to_string()
        }
    }
}

impl VilChain {
    pub fn new() -> Self {
         VilChain::default()
    }

    pub fn get_last_block_hash(&self) -> Option<String> {
        self.last_block_hash.clone()
    }

    pub fn add_block(&mut self, block: Block) {
        self.last_block_hash = Some(block.clone().get_block_hash().clone());
        self.blocks.insert(block.get_block_hash().clone(), block);
        self.chain_size = size_of_val(&self.blocks);
        self.block_count += 1;
    } 
}