use serde::{Deserialize, Serialize};
use std::{collections::HashSet, default::Default, time::SystemTime};

use crate::txn::{publishable_txn::PublishableTransaction, Txn};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockHeaders {
    prev_block_hash: String,
    next_block_hash: Option<String>,
    merkle_root_hash: Option<String>,
    block_size: usize,
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

impl std::ops::Deref for Block {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.headers.merkle_root_hash
    }
}

impl std::iter::Iterator for Block {
    type Item = Txn;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
