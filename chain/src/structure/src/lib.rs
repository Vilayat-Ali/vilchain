use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

#[derive(Serialize, Deserialize)]
pub struct FloatValue {
    pub int_val: u32,
    pub zeroes: u8,
    pub deci_val: u16,
}

impl FloatValue {
    pub fn new(int_val: u32, zeroes: u8, deci_val: u16) -> Self {
        Self {
            int_val,
            zeroes,
            deci_val,
        }
    }
}

impl Display for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}{}",
            self.int_val,
            "0".repeat(self.zeroes as usize),
            self.deci_val
        )
    }
}

impl Debug for FloatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}{}",
            self.int_val,
            "0".repeat(self.zeroes as usize),
            self.deci_val
        )
    }
}
