pub mod Block;
pub mod Txn;

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{default::Default, fmt};

extern crate chrono;

#[derive(Default, Serialize, Deserialize, Hash)]
pub enum FloatValueSign {
    #[default]
    Pos,
    Neg,
}

impl fmt::Display for FloatValueSign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &FloatValueSign::Pos => {
                write!(f, "+")
            }
            &FloatValueSign::Neg => {
                write!(f, "-")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Hash)]
pub struct FloatValue {
    pub sign: FloatValueSign,
    pub int_val: u32,
    pub lead_frac_zeros: u32,
    pub frac_val: u32,
}

impl fmt::Display for FloatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}.{}{}",
            &self.sign.to_string(),
            &self.int_val,
            (0..self.lead_frac_zeros as usize)
                .map(|_x| "0")
                .collect::<String>(),
            &self.frac_val
        )
    }
}

impl Default for FloatValue {
    fn default() -> Self {
        Self {
            sign: FloatValueSign::Pos,
            int_val: 0,
            lead_frac_zeros: 0,
            frac_val: 0,
        }
    }
}

impl FloatValue {
    pub fn new(
        int_val: u32,
        lead_frac_zeros: u32,
        frac_val: u32,
        sign: Option<FloatValueSign>,
    ) -> Self {
        match sign {
            Some(sign_val) => Self {
                sign: sign_val,
                int_val,
                lead_frac_zeros,
                frac_val,
            },
            None => Self {
                sign: FloatValueSign::default(),
                int_val,
                lead_frac_zeros,
                frac_val,
            },
        }
    }
}

pub fn hash_data<T>(data: &T) -> String
where
    T: Hash,
{
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    format!("{:x}", s.finish())
}
