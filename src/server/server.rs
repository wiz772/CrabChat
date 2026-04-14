use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use crate::server::logger;
use crate::shared::protocol::Message;

const LISTENER_ADDR: &str ="127.0.0.1:8080"; 

fn handle_msg(message: String){
    match Message::decode(&message){
        Ok(msg) => {
        
            match msg{

                Message::Chat { content } => {
                    // Vérifier si le client est login
                    // S'il l'est envoyer en broadcast
                    // Sinon void le msg
                }

                Message::Login { user } => {
                    // add client à la hashmap, à faire server state
                }

                Message::Ping => {
                    // renvoyer un pong
                }

            }

        }

        Err(e) => { logger::error(&format!("Couldn't handle the msg: {}", e)); }
    }

}

fn handle_client(mut stream: TcpStream){ 
    let mut buffer = [0; 1024];
    loop {
        let bytes = stream.read(&mut buffer).unwrap();

        if bytes == 0 {
            break; 
        }

        let msg = String::from_utf8_lossy(&buffer[..bytes]);
        handle_msg(msg.to_string() );
    }
}

fn log_connection(stream: &TcpStream){
    match stream.peer_addr() {
        Ok(addr) =>  logger::info(&format!("Connected: {}", addr)),
        Err(e) => logger::error(&format!("Couldn't resolve ip address: {}", e)),
    }
}

fn dispatch_client(stream: TcpStream){
    thread::spawn(move|| {
        handle_client(stream);
    }); 
}

fn handle_incoming_connection(stream: TcpStream){
    log_connection(&stream);
    dispatch_client(stream);
}

fn start_listening(){ 
    let listener = TcpListener::bind(LISTENER_ADDR).unwrap();
    logger::info("Server started.");
    for stream in listener.incoming(){ 
        match stream{
            Ok(stream) => {
                handle_incoming_connection(stream);         
            }

            Err(e) => {
                logger::error(&e.to_string());
            }
        }
    } 
}

fn broadcast(){

}

pub fn init_server_thread(){
    std::thread::spawn(move || {
        start_listening();
    });
}