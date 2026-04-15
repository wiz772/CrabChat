use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::usize;

type Sessions = Arc<Mutex<HashMap<usize, ClientSession>>>;

pub struct ClientSession {
    id: usize,
    stream: TcpStream, 
    username: Option<String>

}

impl ClientSession {

    pub fn new(id: usize, stream: TcpStream) -> Result<Self, String> {
        Ok(ClientSession{ id, stream, username: None})
    }

    pub fn set_username(&mut self, new_username: String) {
        self.username = Some(new_username);
    }

}