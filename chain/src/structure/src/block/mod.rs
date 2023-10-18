use serde::{Deserialize, Serialize};
use std::{collections::HashSet, default::Default, time::SystemTime};

use crate::txn::{publishable_txn::PublishableTransaction, Txn};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeaders {
    prev_block_hash: String,
    next_block_hash: Option<String>,
    merkle_root_hash: Option<String>,
    block_size: usize,
}

impl Default for BlockHeaders {
    fn default() -> Self {
        Self {
            prev_block_hash: String::new(),
            next_block_hash: None,
            merkle_root_hash: None,
            block_size: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block
where
    Txn: PublishableTransaction + Serialize + Deserialize<'static> + std::fmt::Debug,
{
    headers: BlockHeaders,
    txns: HashSet<Txn>,
    timestamp: SystemTime,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            headers: BlockHeaders::default(),
            txns: HashSet::new(),
            timestamp: SystemTime::now(),
        }
    }
}
