use serde::{Deserialize, Serialize};
use std::{default, mem};

use crate::{block::Block, txn::Txn};

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
    fn search_txn(&self, txn_hash: impl Into<String>) -> Option<&Txn>;
    fn search_block(&self, block_hash: impl Into<String>) -> Option<&Block>;
    fn get_genesis_block(&self) -> Option<&Block>;
}

impl Chain for VilChain {
    fn total_size_in_bytes(&self) -> usize {
        mem::size_of_val(&self)
    }

    fn search_txn(&self, txn_hash: impl Into<String>) -> Option<&Txn> {
        todo!()
    }

    fn search_block(&self, block_hash: impl Into<String>) -> Option<&Block> {
        todo!()
    }

    fn get_genesis_block(&self) -> Option<&Block> {
        todo!()
    }
}
