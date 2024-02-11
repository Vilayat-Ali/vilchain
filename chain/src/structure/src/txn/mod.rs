use serde::{Serialize, Deserialize};
use std::fmt::{Debug, Display};

use super::FloatValue;

#[derive(Serialize, Deserialize)]
pub struct Txn {
    from: String,
    to: String,
    value: FloatValue,
    gas: FloatValue
}

impl Display for Txn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "{} --> {} ~ {} @ {}",
        &self.from, &self.to, &self.value.to_string(), &self.gas.to_string())
    }
}

impl Debug for Txn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "{} --> {} ~ {} @ {}",
        &self.from, &self.to, &self.value.to_string(), &self.gas.to_string())
    }
}

impl Txn {
    pub fn new<S>(from:S, to: S, value: FloatValue, gas: FloatValue) -> Self 
        where S: Into<String> {
            Self {
                from: from.into(),
                to: to.into(),
                value: value.into(),
                gas: gas.into()
            }
        }
}