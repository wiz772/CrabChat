use std::net::TcpStream;
use std::io::Result;

use crate::shared::protocol::ClientMessage;

pub struct Client {
    stream: TcpStream,
    username: String
}
 
impl Client{

    pub fn new(address: &str, username: String) -> Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Client { stream, username })
    }

    pub fn send(&mut self, msg: ClientMessage) -> Result<()> {

        Ok(())
    }

}