use serde::{Serialize, Deserialize};
use chrono::Local;
use serde_json::to_string_pretty;
use sha3::{Digest, Sha3_512};

use crate::BigNum;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Txn {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: BigNum,
    pub timestamp: String,
}

impl Txn {
    pub fn to_json(&self) -> String {
        to_string_pretty(&self).unwrap()
    }

    pub fn compute_hash(&self) -> String {
        let placeholder_txn = PlaceholderUnPublishedTxn {
            from: self.from.clone(),
            to: self.to.clone(),
            value: self.value.clone(),
            timestamp: self.timestamp.clone(),
        };

        let mut hasher = Sha3_512::new();
        hasher.update(serde_json::to_vec(&placeholder_txn).unwrap());

        format!("{:x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize)]
struct PlaceholderUnPublishedTxn {
    from: String,
    to: String,
    value: BigNum,
    timestamp: String,
}

impl From<UnPublishedTxn> for Txn {
    fn from(txn: UnPublishedTxn) -> Self {
        let timestamp = Local::now().to_string();

        let placeholder_txn = PlaceholderUnPublishedTxn {
            from: txn.from.clone(),
            to: txn.to.clone(),
            value: txn.value.clone(),
            timestamp: timestamp.clone(),
        };

        let mut hasher = Sha3_512::new();
        hasher.update(serde_json::to_vec(&placeholder_txn).unwrap());
        
        Self {
            hash: format!("{:x}", hasher.finalize()),
            from: txn.from,
            to: txn.to,
            value: txn.value,
            timestamp,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnPublishedTxn {
    pub from: String,
    pub to: String,
    pub value: BigNum,
}

impl UnPublishedTxn {
    pub fn new<S>(from: S, to: S, value: S) -> Result<Self, String> 
        where S: Into<String>,
        {
            let value = BigNum::from_string(value)?;

            Ok(Self {
                from: from.into(),
                to: to.into(),
                value
            })
    }

    pub fn to_json(&self) -> String {
        to_string_pretty(&self).unwrap()
    }
}