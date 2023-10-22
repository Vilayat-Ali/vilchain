use dotenv::dotenv;
use pretty_env_logger::init as initiateLogger;
use std::{env as stdEnv, io};
use uuid::Uuid;

extern crate log;

use node::{
    env::{get_envs, ENV},
    utils::display_banner_on_startup,
};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // setting up envs and loggers
    dotenv().ok();
    initiateLogger();
    let env_var: ENV = get_envs();
    let node_id: String = Uuid::new_v4().to_string();

    let _env_vec: stdEnv::Args = stdEnv::args();

    let start_server = move || {
        display_banner_on_startup(&node_id, &env_var);
    };

    start_server();

    Ok(())
}
