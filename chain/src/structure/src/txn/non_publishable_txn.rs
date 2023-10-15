use serde::{Deserialize, Serialize};

use super::{publishable_txn::PublishableTransaction, Txn};
use bcrypt::{hash, verify, BcryptResult};

pub trait NonPublishableTransaction: Serialize + Deserialize<'static> + std::fmt::Debug {
    fn compute_hash(&self) -> BcryptResult<String>;
    fn validate_hash(&self) -> bool;
    // fn publish<T>(&mut self) -> BcryptResult<impl PublishableTransaction>
    // where
    //     T: PublishableTransaction + Serialize + std::fmt::Debug;
}

impl NonPublishableTransaction for Txn {
    fn compute_hash(&self) -> BcryptResult<String> {
        let hash: String = hash(
            format!(
                "{}-{}-{}-{:#?}-{:#?}",
                self.from, self.to, self.value, self.timestamp, self.fee
            ),
            2552,
        )?;
        Ok(hash)
    }

    fn validate_hash(&self) -> bool {
        let raw_val: String = format!(
            "{}-{}-{}-{:#?}-{:#?}",
            self.from, self.to, self.value, self.timestamp, self.fee
        );

        verify(raw_val, self.hash.clone().unwrap().as_ref()).unwrap()
    }

    // fn publish<T>(&mut self) -> BcryptResult<impl PublishableTransaction>
    // where
    //     T: PublishableTransaction + Serialize + std::fmt::Debug,
    // {
    //     let hash: String = self.compute_hash()?;
    //     Ok(Self {
    //         hash: Some(hash),
    //         from: self.from.clone(),
    //         to: self.to.clone(),
    //         value: self.value.clone(),
    //         fee: self.fee.clone(),
    //         timestamp: self.timestamp,
    //     })
    // }
}
