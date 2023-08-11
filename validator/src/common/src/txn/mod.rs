pub mod builder;

use crate::FloatValue;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Hash, Debug)]
pub struct Txn {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: FloatValue,
    pub gas: FloatValue,
    pub time_stamp: String,
}


pub trait Transaction {}
