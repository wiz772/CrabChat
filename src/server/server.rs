use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use crate::server::logger;
use crate::shared::protocol::ClientMessage;
use crate::server::client_session::{ClientSession, Sessions, Session};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


const LISTENER_ADDR: &str ="127.0.0.1:8080"; 

fn disconnect_client(client_session: Session, sessions: Sessions){}


fn handle_msg(client_session: Session, message: String, sessions: Sessions){
    match ClientMessage::decode(&message){
        Ok(msg) => {
        
            match msg{

                ClientMessage::Chat { content } => {
                    // Vérifier si le client est login
                    // S'il l'est envoyer en broadcast
                    // Sinon void le msg
                }

                ClientMessage::Login { user } => {
                    // add client à la hashmap, à faire
                }

                ClientMessage::Ping => {
                    // renvoyer un pong
                }

            }

        }

        Err(e) => { logger::error(&format!("Couldn't handle the msg: {}", e)); }
    }

}

fn handle_client(client_session: Session, sessions: Sessions){ 
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {

        let bytes = {
            let mut session = client_session.lock().unwrap();
            session.read(&mut buffer).unwrap()
        };
        if bytes == 0 {
            disconnect_client(client_session: Session, sessions: Sessions);
            break;
        }

        let msg = String::from_utf8_lossy(&buffer[..bytes]);
        handle_msg( client_session.clone(), msg.to_string(), sessions.clone());
    }
}

fn log_connection(stream: &TcpStream){
    match stream.peer_addr() {
        Ok(addr) =>  logger::info(&format!("Connected: {}", addr)),
        Err(e) => logger::error(&format!("Couldn't resolve ip address: {}", e)),
    }
}

fn generate_id(sessions: &Sessions) -> usize {
    let s = sessions.lock().unwrap();
    let id = s.len() + 1;
    id
}

fn create_client_session(stream: TcpStream, sessions: &Sessions) -> (usize, Arc<Mutex<ClientSession>>){
    let id = generate_id(&sessions);

    let client_session = Arc::new(Mutex::new(
        ClientSession::new(id, stream).unwrap() 
    ));

    (id, client_session)
}

fn insert_client_session_in_global_sessions(id: usize, client_session: Session, sessions: &Sessions){
    sessions.lock().unwrap().insert(id, client_session);
    logger::info("Added a  client_session");
}

fn dispatch_client(stream: TcpStream, sessions: Sessions){
    let (id, client_session) = create_client_session(stream, &sessions);

    insert_client_session_in_global_sessions(id, client_session.clone(), &sessions);

    thread::spawn(move || {
        handle_client(client_session, sessions);
    });
}

fn handle_incoming_connection(stream: TcpStream, sessions: Sessions){
    log_connection(&stream);
    dispatch_client(stream, sessions);
}

fn start_listening(sessions: Sessions){ 
    let listener = TcpListener::bind(LISTENER_ADDR).unwrap();
    logger::info("Server started.");
    for stream in listener.incoming(){ 
        match stream{
            Ok(stream) => {
                handle_incoming_connection(stream, sessions.clone());         
            }

            Err(e) => {
                logger::error(&e.to_string());
            }
        }
    } 
}

fn broadcast(sessions: Sessions, message: String){}


pub fn init_server_thread(){
    let sessions: Sessions = Arc::new(Mutex::new(HashMap::new()));
    thread::spawn(move || {
        start_listening(sessions);
    });
}