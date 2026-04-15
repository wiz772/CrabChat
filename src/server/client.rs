use std::net::TcpStream;
pub struct Client {
    stream: TcpStream,
    username: String
}