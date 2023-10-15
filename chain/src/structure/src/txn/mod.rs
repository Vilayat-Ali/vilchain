use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::FloatValue;

pub trait NonPublishedTransaction {}

pub trait PublishedTransaction {}

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Txn {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: FloatValue,
    pub timestamp: SystemTime,
    pub fee: FloatValue,
}

impl Txn {
    pub fn compute_hash(&self) -> String {
        String::new()
    }
}

pub struct TxnBuilder {
    pub hash: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: FloatValue,
    pub timestamp: Option<SystemTime>, // time when submitted for validation
}

impl TxnBuilder {
    pub fn builder() -> Self {
        TxnBuilder {
            hash: None,
            from: None,
            to: None,
            value: FloatValue::default(),
            timestamp: None,
        }
    }

    pub fn set_from(&mut self, from: String) -> &mut Self {
        self.from = Some(from);
        self
    }

    pub fn set_to(&mut self, to: String) -> &mut Self {
        self.to = Some(to);
        self
    }

    pub fn set_value(&mut self, value: FloatValue) -> &mut Self {
        self.value = value;
        self
    }
}
