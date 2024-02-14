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

pub trait TxnList {
    fn compute_merkle_hash(txn_list: Vec<Txn>) -> String;
}

impl TxnList for Vec<Txn> {
    fn compute_merkle_hash(txn_list: Vec<Txn>) -> String {
        let combine_hash = |txn_1: &Txn, txn_2: &Txn| {
            let hash_txn_1 = txn_1.hash.clone();
            let concat_hash = hash_txn_1 + &(txn_2.hash);

            let mut hasher = Sha3_512::new();
            hasher.update(serde_json::to_vec(&concat_hash).unwrap());
            
            format!("{:x}", hasher.finalize())
        };

        if txn_list.len() == 0 {
            return String::new();
        }

        if txn_list.len() == 1 {
            return combine_hash(&txn_list[0], &txn_list[0]);
        }

        if txn_list.len() == 2 {
            return combine_hash(&txn_list[0], &txn_list[1]);
        }

        let mid = txn_list.len() / 2;
        let left_hash_vec = txn_list[0..mid].to_vec();
        let right_hash_vec = txn_list[mid..].to_vec();

        let left_hash = Vec::compute_merkle_hash(left_hash_vec);
        let right_hash = Vec::compute_merkle_hash(right_hash_vec);

        let mut hasher = Sha3_512::new();
        hasher.update((left_hash + &right_hash).as_bytes());
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