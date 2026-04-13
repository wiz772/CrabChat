use std::net::{TcpListener, TcpStream};
use std::thread;
const LISTENER_ADDR: &str ="127.0.0.1:8080"; 

fn handle_client(mut client: TcpStream){ 

} 

fn start_listening(){ 
    let listener = TcpListener::bind(LISTENER_ADDR).unwrap();
    println!("Server started."); 
    for stream in listener.incoming(){ 
        match stream{
            Ok(mut stream) => {
                let ip_addr = stream.peer_addr().unwrap();
                println!("Connected: {}", ip_addr);

                // créer thread par client
                thread::spawn(move ||{
                    handle_client(stream);
                });            
            }

            Err(e) => {
                println!("[ERROR] {}", e)
            }
        }
    } 
}

fn main() {
    start_listening();
}
