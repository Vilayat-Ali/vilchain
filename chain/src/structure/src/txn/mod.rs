pub mod non_publishable_txn;
pub mod publishable_txn;

use super::txn::non_publishable_txn::NonPublishableTransaction;
use crate::FloatValue;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::{hash::Hash, time::SystemTime};

#[derive(Serialize, Deserialize, Hash, Clone, Debug)]
pub struct Txn {
    pub hash: Option<String>,
    pub from: String,
    pub to: String,
    pub value: FloatValue,
    pub fee: Option<FloatValue>,
    pub timestamp: SystemTime,
}

impl PartialEq for Txn {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Txn {}

impl Txn {
    pub fn compute_hash(&self) -> String {
        let struct_bytes: Vec<u8> = serde_json::to_vec(&self).unwrap();
        let mut hasher = Keccak256::new();
        hasher.update(struct_bytes);
        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);
        format!("0x{}", hash_hex)
    }

    pub fn verify_hash(&self, hash: String) -> bool {
        println!("{}", self.compute_hash());
        println!("{}", self.compute_hash());

        self.compute_hash() == hash
    }
}

#[derive(Default)]
pub struct TxnBuilder {
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: FloatValue,
}

impl TxnBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_from(&mut self, from: impl Into<String>) -> &mut Self {
        self.from = Some(from.into());
        self
    }

    pub fn set_to(&mut self, to: impl Into<String>) -> &mut Self {
        self.to = Some(to.into());
        self
    }

    pub fn set_value(&mut self, value: FloatValue) -> &mut Self {
        self.value = value;
        self
    }

    pub fn build<'a>(&'a mut self) -> Result<Txn, String>
    where
        Txn: NonPublishableTransaction + Serialize + Deserialize<'a> + std::fmt::Debug,
    {
        let txn = Txn {
            hash: None,
            from: self.from.clone().take().unwrap(),
            to: self.to.clone().take().unwrap(),
            value: self.value.clone(),
            fee: None,
            timestamp: SystemTime::now(),
        };

        Ok(txn)
    }
}
