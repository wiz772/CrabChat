use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::usize;
use std::io::{Read, Write};



pub type Session = Arc<Mutex<ClientSession>>;
pub type Sessions = Arc<Mutex<HashMap<usize, Session>>>;

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

    pub fn is_logged_in(&self) -> bool {
        self.username.is_some()
    }

    pub fn send(&mut self, msg: &str) -> std::io::Result<()> {
        self.stream.write_all(format!("{}\n", msg).as_bytes())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }

    pub fn write(&mut self, msg: &str) -> std::io::Result<()> {
        self.stream.write_all(msg.as_bytes())
    }

}