use serde::{Deserialize, Serialize};

use super::{publishable_txn::PublishableTransaction, Txn};

pub trait NonPublishableTransaction {
    fn publish<'a>(&mut self) -> Self
    where
        Self: PublishableTransaction + Serialize + Deserialize<'a> + std::fmt::Debug;
}

impl NonPublishableTransaction for Txn {
    fn publish<'a>(&mut self) -> Self
    where
        Self: PublishableTransaction + Serialize + Deserialize<'a> + std::fmt::Debug,
    {
        println!("!!=> {}", self.compute_hash());
        self.hash = Some(self.compute_hash());
        self.clone()
    }
}
