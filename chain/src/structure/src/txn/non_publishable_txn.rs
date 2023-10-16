use super::{publishable_txn::PublishableTransaction, Txn};

pub trait NonPublishableTransaction {
    fn publish(&mut self) -> Box<dyn PublishableTransaction>;
}

impl NonPublishableTransaction for Txn {
    fn publish(&mut self) -> Box<dyn PublishableTransaction> {
        todo!()
    }
}
