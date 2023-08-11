use super::Block;
use crate::txn::Txn;
use chrono::{DateTime, Utc};

const BLOCK_TXN_LIMIT: usize = 6;

pub struct BlockBuilder {
    pub mined_by: Option<String>,
    pub txns: Vec<Txn>,
    pub time_stamp: Option<String>,
}

struct PlaceholderBlock {
    mined_by: String,
    txns: Vec<Txn>,
    time_stamp: String,
}

impl BlockBuilder {
    pub fn init() -> Self {
        Self {
            mined_by: None,
            txns: Vec::with_capacity(BLOCK_TXN_LIMIT),
            time_stamp: None,
        }
    }

    pub fn is_txn_limit_reached(&self) -> bool {
        &self.txns.len() >= &BLOCK_TXN_LIMIT
    }

    pub fn set_mined_by(&mut self, validator_id: String) -> &mut Self {
        self.mined_by = Some(validator_id);
        self
    }

    pub fn add_txn(&mut self, txn: Txn) -> &mut Self {
        if !self.is_txn_limit_reached() {
            self.txns.push(txn);
        }
        self
    }

    pub fn build(&mut self) -> Block {
        let utc: DateTime<Utc> = Utc::now();
        Block {
            parent_hash: String::new(),
            mined_by: String::new(),
            txns: Vec::new(),
            time_stamp: utc.to_string(),
        }
    }
}
