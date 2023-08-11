pub mod block;
pub mod txn;

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::{default::Default, fmt};

extern crate chrono;

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct FloatValue {
    pub int_val: u16,
    pub lead_frac_zeros: u16,
    pub frac_val: u16,
}

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}{}",
            &self.int_val,
            "0".repeat(self.lead_frac_zeros as usize),
            &self.frac_val
        )
    }
}

impl Default for FloatValue {
    fn default() -> Self {
        Self {
            int_val: 0,
            lead_frac_zeros: 0,
            frac_val: 0,
        }
    }
}

impl Debug for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}{}",
            &self.int_val,
            "0".repeat(self.lead_frac_zeros as usize),
            &self.frac_val
        )
    }
}

impl FloatValue {
    pub fn new(int_val: u16, lead_frac_zeros: u16, frac_val: u16) -> Self {
        Self {
            int_val,
            lead_frac_zeros,
            frac_val,
        }
    }
}

pub fn hash_data<T>(data: &T) -> String
where
    T: Hash,
{
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    format!("0x{:x}", s.finish())
}
