use std::thread;
use std::time::Duration;
use crate::client::client::Client;
use crate::server::server::init_server_thread;

const LISTENER_ADDR: &str ="127.0.0.1:8080"; 


fn main() {
    init_server_thread();

    thread::sleep(Duration::from_millis(100));

    let mut client1 = Client::new(LISTENER_ADDR, "gougou".to_string()).unwrap();
    // client1.write("la famax krili").unwrap();


    std::thread::sleep(std::time::Duration::from_secs(10));
}


mod client;
mod server;
mod shared;