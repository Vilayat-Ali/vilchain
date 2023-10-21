// use std::{
//     collections::HashMap,
//     env,
//     io::Error as IoError,
//     net::SocketAddr,
//     sync::{Arc, Mutex},
// };

// use uuid::Uuid;

// use futures_channel::mpsc::{unbounded, UnboundedSender};
// use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

// use tokio::net::{TcpListener, TcpStream};
// use tokio_tungstenite::tungstenite::protocol::Message;

// type Tx = UnboundedSender<Message>;
// type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

// async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
//     println!("Incoming TCP connection from: {}", addr);

//     let ws_stream = tokio_tungstenite::accept_async(raw_stream)
//         .await
//         .expect("Error during the websocket handshake occurred");
//     println!("WebSocket connection established: {}", addr);

//     // Insert the write part of this peer to the peer map.
//     let (tx, rx) = unbounded();
//     peer_map.lock().unwrap().insert(addr, tx);

//     let (outgoing, incoming) = ws_stream.split();

//     let broadcast_incoming = incoming.try_for_each(|msg| {
//         println!(
//             "Received a message from {}: {}",
//             addr,
//             msg.to_text().unwrap()
//         );
//         let peers = peer_map.lock().unwrap();

//         // We want to broadcast the message to everyone except ourselves.
//         let broadcast_recipients = peers
//             .iter()
//             .filter(|(peer_addr, _)| peer_addr != &&addr)
//             .map(|(_, ws_sink)| ws_sink);

//         for recp in broadcast_recipients {
//             recp.unbounded_send(msg.clone()).unwrap();
//         }

//         future::ok(())
//     });

//     let receive_from_others = rx.map(Ok).forward(outgoing);

//     pin_mut!(broadcast_incoming, receive_from_others);
//     future::select(broadcast_incoming, receive_from_others).await;

//     println!("{} disconnected", &addr);
//     peer_map.lock().unwrap().remove(&addr);
// }

// #[tokio::main]
// async fn main() -> Result<(), IoError> {
//     dotenv::dotenv().ok();
//     pretty_env_logger::init();

//     let addr = env::args()
//         .nth(1)
//         .unwrap_or_else(|| "127.0.0.1:8080".to_string());

//     let state: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>> =
//         PeerMap::new(Mutex::new(HashMap::new()));

//     let node_id: String = Uuid::new_v4().to_string();
//     let try_socket: Result<TcpListener, IoError> = TcpListener::bind(&addr).await;
//     let listener = try_socket.expect("Failed to bind");

//     println!("Node ID - {}", node_id);

//     while let Ok((stream, addr)) = listener.accept().await {
//         tokio::spawn(handle_connection(state.clone(), stream, addr));
//     }

//     Ok(())
// }

use structure::{
    block::Block,
    txn::{non_publishable_txn::NonPublishableTransaction, TxnBuilder},
    FloatValue,
};

fn main() {
    let mut b: Block = Block::default();
    for x in 0..15 {
        let mut txn = TxnBuilder::new()
            .set_from(x.to_string().repeat(5))
            .set_to(x.to_string().repeat(5))
            .set_value(FloatValue::default())
            .build()
            .unwrap();

        b.insert_txn(txn.publish()).unwrap();

        println!("{}", b.compute_merkle_root_hash());

        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
