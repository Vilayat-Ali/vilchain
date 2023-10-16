pub mod non_publishable_txn;
pub mod publishable_txn;

use std::time::SystemTime;

use super::txn::non_publishable_txn::NonPublishableTransaction;
use crate::FloatValue;
use bcrypt::{hash, verify, BcryptResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Txn {
    hash: Option<String>,
    from: String,
    to: String,
    value: FloatValue,
    fee: Option<FloatValue>,
    timestamp: SystemTime,
}

impl Txn {
    pub fn compute_hash(&self) -> BcryptResult<String> {
        let hash: String = hash(
            format!(
                "{}-{}-{}-{:#?}",
                &self.from, &self.to, &self.value, &self.timestamp
            ),
            225,
        )?;
        Ok(hash)
    }

    pub fn verify_hash(&self, hash: &str) -> BcryptResult<bool> {
        verify(hash, self.hash.clone().unwrap().as_ref())
    }
}

pub struct TxnBuilder {
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: FloatValue,
}

impl TxnBuilder {
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
            value: FloatValue::default(),
        }
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
