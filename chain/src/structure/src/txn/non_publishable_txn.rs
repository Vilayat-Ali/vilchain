use crate::FloatValue;

use super::{publishable_txn::PublishableTransaction, Txn};

pub trait NonPublishableTransaction {}

impl NonPublishableTransaction for Txn {}
