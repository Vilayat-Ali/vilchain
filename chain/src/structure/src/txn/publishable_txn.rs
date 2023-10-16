use super::Txn;

pub trait PublishableTransaction {}

impl PublishableTransaction for Txn {}
