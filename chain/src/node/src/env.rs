use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use envy::from_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct ENV {
    pub rust_log: String,
    pub addr: String,
}

pub fn get_envs() -> ENV {
    dotenv().ok();
    match from_env() {
        Ok(env) => env,
        Err(_) => panic!("ENV Error: Envs improperly configured!"),
    }
}
