use super::Txn;
use crate::{hash_data, FloatValue};
use chrono::prelude::*;

pub struct TxnBuilder {
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: Option<FloatValue>,
    pub gas: Option<FloatValue>,
}

#[derive(Hash)]
struct PlaceholderTxn {
    from: String,
    to: String,
    value: FloatValue,
    gas: FloatValue,
    time_stamp: String,
}

impl TxnBuilder {
    pub fn init() -> Self {
        Self {
            from: None,
            to: None,
            value: None,
            gas: None,
        }
    }

    pub fn set_sender_address(&mut self, address: String) -> &mut Self {
        self.from = Some(address);
        self
    }

    pub fn set_recepient_address(&mut self, address: String) -> &mut Self {
        self.to = Some(address);
        self
    }

    pub fn set_value(&mut self, value: FloatValue) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub fn set_gas(&mut self, gas: FloatValue) -> &mut Self {
        self.gas = Some(gas);
        self
    }

    pub fn build(&mut self) -> Result<Txn, &'static str> {
        if self.from.is_none() || self.to.is_none() || self.value.is_none() || self.gas.is_none() {
            Err("TxnBuilder Error: Some value is none")
        } else {
            let utc: DateTime<Utc> = Utc::now();

            let p_txn: PlaceholderTxn = PlaceholderTxn {
                from: self.from.clone().take().unwrap(),
                to: self.to.clone().take().unwrap(),
                value: self.value.clone().take().unwrap(),
                gas: self.gas.clone().take().unwrap(),
                time_stamp: utc.to_string(),
            };

            Ok(Txn {
                hash: hash_data(&p_txn),
                from: self.from.take().unwrap(),
                to: self.to.take().unwrap(),
                value: self.value.take().unwrap(),
                gas: self.gas.take().unwrap(),
                time_stamp: utc.to_string(),
            })
        }
    }
}
