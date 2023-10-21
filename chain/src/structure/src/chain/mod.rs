use serde::{Deserialize, Serialize};
use std::{default, mem};

use crate::block::Block;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainOptions {
    pub block_txn_limit: usize,
    pub max_block_size: usize,
    pub min_block_size: usize,
}

impl default::Default for ChainOptions {
    fn default() -> Self {
        Self {
            block_txn_limit: 15,
            max_block_size: 200,
            min_block_size: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VilChain {
    pub block_count: usize,
    pub blocks: Box<Block>,
}

impl VilChain {
    pub fn new() -> Self {
        Self {
            block_count: 1,
            blocks: Box::new(Block::default()),
        }
    }
}

pub trait Chain {
    fn total_size_in_bytes(&self) -> usize;
}

impl Chain for VilChain {
    fn total_size_in_bytes(&self) -> usize {
        mem::size_of_val(&self)
    }
}
