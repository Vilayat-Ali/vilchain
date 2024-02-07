use envy::from_env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ENV {
    pub rust_log: String,
    pub port: u16,
}

impl ENV {
    pub fn new() -> ENV {
        // unwrap is a safe play here as application should crash incase of discrepancy in environment
        // variables.
        from_env().unwrap()
    }
}
