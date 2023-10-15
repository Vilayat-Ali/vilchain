use serde::{Deserialize, Serialize};

use super::Txn;

pub trait PublishableTransaction: Serialize + Deserialize<'static> + std::fmt::Debug {}

impl PublishableTransaction for Txn {}
