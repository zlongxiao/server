extern crate env_logger;
extern crate mio_extras;
extern crate time;

use server::db::connect;
use server::ws_server::ws_server;
use server::api::api;


#[tokio::main]
async fn main() {
    // Setup logging
    env_logger::init();

    // Run the WebSocket
    // ws_server();
    // connect().unwrap();
    api().await;
}
