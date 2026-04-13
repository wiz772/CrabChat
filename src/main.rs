use std::net::{TcpListener, TcpStream};
use std::thread;
const LISTENER_ADDR: &str ="127.0.0.1:8080"; 

fn handle_client(mut client: TcpStream){ 

}

fn display_connection_log(stream: &TcpStream){
    match stream.peer_addr() {
        Ok(addr) =>  println!("Connected: {}", addr),
        Err(e) => println!("[ERROR] Could not get client address: {}", e),
    }
}

fn dispatch_client(mut stream: TcpStream){
    thread::spawn(move|| {
        handle_client(stream);
    }); 
}

fn handle_incoming_connection(mut stream: TcpStream){
    display_connection_log(&stream);
    dispatch_client(stream);
}

fn start_listening(){ 
    let listener = TcpListener::bind(LISTENER_ADDR).unwrap();
    println!("Server started."); 
    for stream in listener.incoming(){ 
        match stream{
            Ok(stream) => {
                handle_incoming_connection(stream);         
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
