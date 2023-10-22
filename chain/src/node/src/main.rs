use dotenv::dotenv;
use futures_channel::mpsc::UnboundedSender;
use pretty_env_logger::init as initiateLogger;
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use std::{env as stdEnv, io};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::Uuid;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

extern crate log;

use node::{
    env::{get_envs, ENV},
    utils::display_banner_on_startup,
    ws::handle_connection,
};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // setting up envs and loggers
    dotenv().ok();
    initiateLogger();
    let env_var: ENV = get_envs();
    let node_id: String = Uuid::new_v4().to_string();
    let addr: String = format!(
        "{}:{}",
        env_var.addr,
        env::args().nth(1).unwrap_or_else(|| "8000".to_string())
    );

    let _env_vec: stdEnv::Args = stdEnv::args();

    let start_server = move || async move {
        display_banner_on_startup(&node_id, &env_var);

        let state = PeerMap::new(Mutex::new(HashMap::new()));

        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        log::info!("Listening on: {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(state.clone(), stream, addr));
        }
    };

    start_server().await;

    Ok(())
}
