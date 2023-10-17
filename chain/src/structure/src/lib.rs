pub mod block;
pub mod chain;
pub mod txn;

use serde::{Deserialize, Serialize};
use std::{default, fmt};

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct FloatValue {
    pub int_val: u32,
    pub zero_count: usize,
    pub frac_val: u32,
}

impl FloatValue {
    pub fn new(int_val: u32, zero_count: usize, frac_val: u32) -> Self {
        Self {
            int_val,
            zero_count,
            frac_val,
        }
    }
}

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}{}",
            self.int_val,
            "0".repeat(self.zero_count),
            self.frac_val
        )
    }
}

impl default::Default for FloatValue {
    fn default() -> Self {
        // default value => 0.00001
        FloatValue::new(0, 4, 1)
    }
}
