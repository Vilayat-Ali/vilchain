pub mod builder;

use super::{txn::Txn, FloatValue};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct Block {
    pub parent_hash: String,
    pub mined_by: String,
    pub txns: Vec<Txn>,
    pub time_stamp: String,
}

pub trait TxnBlock {
    fn computer_txn_merkle_hash(&self) -> String;
}
