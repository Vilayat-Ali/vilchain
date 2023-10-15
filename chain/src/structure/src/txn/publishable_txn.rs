use bcrypt::{hash, verify, BcryptResult};

use super::Txn;

pub trait PublishableTransaction {
    fn compute_hash(&self) -> BcryptResult<String>;
    fn validate_hash(&self) -> bool;
    fn publish(&mut self) -> BcryptResult<Self>
    where
        Self: Sized;
}

impl PublishableTransaction for Txn {
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

    fn publish(&mut self) -> BcryptResult<Self> {
        let hash: String = self.compute_hash()?;
        Ok(Self {
            hash: Some(hash),
            from: self.from.clone(),
            to: self.to.clone(),
            value: self.value.clone(),
            fee: self.fee.clone(),
            timestamp: self.timestamp,
        })
    }
}
