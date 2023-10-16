extern crate log;
extern crate pretty_env_logger;
extern crate ws;

use dotenv;

use ws::listen;

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    if let Err(error) = listen("127.0.0.1:3012", |out| {
        move |msg| {
            println!("Server got message '{}'. ", msg);
            out.send(msg)
        }
    }) {
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
