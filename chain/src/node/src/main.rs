extern crate log;
extern crate pretty_env_logger;

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}
