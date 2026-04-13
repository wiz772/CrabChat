use std::io::Write;
use std::net::TcpStream;
use std::io::Result;

pub struct Client {
    stream: TcpStream,
    username: String
}
 
impl Client{

    pub fn new(address: &str, username: String) -> Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Client { stream, username })
    }

    pub fn write(&mut self, msg: &str) -> Result<()> {
        let full_msg = format!("{}: {}", self.username, msg);
        self.stream.write_all(full_msg.as_bytes())?;
        Ok(())
    }

}