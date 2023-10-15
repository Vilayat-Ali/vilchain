pub mod non_publishable_txn;
pub mod publishable_txn;

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use self::non_publishable_txn::NonPublishableTransaction;
use crate::FloatValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Txn {
    pub hash: Option<String>,
    pub from: String,
    pub to: String,
    pub value: FloatValue,
    pub timestamp: SystemTime,
    pub fee: Option<FloatValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxnBuilder {
    pub hash: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: Option<FloatValue>,
    pub fee: Option<FloatValue>,
}

impl TxnBuilder {
    pub fn new() -> Self {
        Self {
            hash: None,
            from: None,
            to: None,
            value: None,
            fee: None,
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
        self.value = Some(value);
        self
    }

    pub fn build(&mut self) -> impl NonPublishableTransaction {
        Txn {
            hash: None,
            from: self
                .from
                .clone()
                .take()
                .expect("Txn Builder Error: Unpublishable txn builder cannot resolve 'from'"),
            to: self
                .to
                .clone()
                .take()
                .expect("Txn Builder Error: Unpublishable txn builder cannot resolve 'to'"),
            value: self
                .value
                .clone()
                .take()
                .expect("Txn Builder Error: Unpublishable txn builder cannot resolve 'value'"),
            timestamp: SystemTime::now(),
            fee: None,
        }
    }
}
