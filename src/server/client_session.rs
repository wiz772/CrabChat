use std::net::TcpStream;

pub struct ClientSession {

    stream: TcpStream, 
    username: Option<String>

}

// impl Client {

// }