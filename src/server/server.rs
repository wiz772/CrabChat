    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::io::Read;
    use crate::server::logger;


    const LISTENER_ADDR: &str ="127.0.0.1:8080"; 

    fn handle_client(mut client: TcpStream){ 
        let mut buffer = [0; 1024];
        loop {
            let bytes = client.read(&mut buffer).unwrap();

            if bytes == 0 {
                break; 
            }

            let msg = String::from_utf8_lossy(&buffer[..bytes]);
            println!("  - {}", msg);
        }
    }

    fn display_connection_log(stream: &TcpStream){
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
        display_connection_log(&stream);
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