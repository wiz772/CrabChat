use crate::server::server::init_server_thread;

fn main() {
    init_server_thread();
}


mod client;
mod server;
mod shared;