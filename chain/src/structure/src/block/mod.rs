use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default::Default, time::SystemTime};

use crate::txn::{publishable_txn::PublishableTransaction, Txn};
use utils::hash;

const BLOCK_TXN_SIZE: usize = 15;

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
    txns: HashMap<String, Txn>,
    timestamp: SystemTime,
}

impl Block {
    fn compute_hash(hash_list: Vec<String>) -> String {
        let window_size: usize = hash_list.len() / 2;
        let have_left_over: bool = hash_list.len() as f64 % 2_f64 != 0_f64;
        let mut stack: Vec<String> = Vec::with_capacity(window_size);

        if have_left_over {
            todo!()
        } else {
            for idx in 0..window_size {
                let level_hash: String =
                    hash::compute_hash(format!("{}{}", hash_list[idx], hash_list[idx + 1]));

                stack.push(level_hash);
            }
        }

        todo!()
    }

    fn compute_merkle_root_hash(&self) -> String {
        let hash_list: Vec<String> = self
            .txns
            .iter()
            .map(|txn| {
                let txn_instance: Txn = txn.1.clone();
                txn_instance.hash.unwrap()
            })
            .collect::<Vec<String>>();

        Self::compute_hash(hash_list)
    }

    pub fn insert_txn(&mut self, txn: Txn) -> Result<(), &'static str> {
        if self.txns.len() < BLOCK_TXN_SIZE {
            // we are 100% sure that we will be having a hash as the txn struct implements PublishableTransaction trait
            self.txns.insert(txn.hash.clone().unwrap(), txn);
            self.compute_merkle_root_hash(); // updating merkle root hash
            Ok(())
        } else {
            Err("Txn Limit reached")
        }
    }

    pub fn get_txn_count(&self) -> usize {
        self.txns.len()
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            headers: BlockHeaders::default(),
            txns: HashMap::with_capacity(BLOCK_TXN_SIZE),
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
