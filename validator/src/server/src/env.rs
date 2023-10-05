extern crate dotenv;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ENV {
    pub port: u16,
}

pub fn provide_env() -> ENV {
    dotenv().ok();

    match envy::from_env::<ENV>() {
        Ok(env) => env,
        Err(e) => panic!("{}", e),
    }
}
