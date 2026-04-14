pub enum ClientMessage {

    Chat { content: String },
    Login { user: String },
    Ping,

}

impl ClientMessage {

    pub fn encode(&self) -> String {

        match self {

            ClientMessage::Chat { content } => format!("CHAT|{}\n", content),
            ClientMessage::Login { user } => format!("LOGIN|{}\n", user),
            ClientMessage::Ping => "PING\n".to_string()

        }

    }

    pub fn decode(input: &str) -> Result<ClientMessage, String> {
        let input = input.trim();

        let mut parts = input.splitn(2, '|');
        let kind = parts.next().ok_or("ClientMessage: Malformed protocol message, missing kind.")?;
        let content = parts.next();
        match kind {
            "CHAT" => {
                let content = content.ok_or("Missing chat content.")?;
                Ok(ClientMessage::Chat { content: content.to_string() })
            }

            "LOGIN" => {
                let content = content.ok_or("ClientMessage: Missing login content.")?;
                Ok(ClientMessage::Login { user: content.to_string() })
            }

            "PING" => {
                Ok(ClientMessage::Ping)
            }
            _ => Err("ClientMessage: Unknown ClientMessage type".into())
        }
    }
}

pub enum ServerMessage {

    LoginOk { username: String },
    LoginErr { error: String },
    Pong

}

impl ServerMessage {


    pub fn encode(&self) -> String {

        match self {

            ServerMessage::LoginOk { username } => format!("LOGIN_OK|{}\n", username),
            ServerMessage::LoginErr { error } => format!("LOGIN_ERROR|{}\n", error),
            ServerMessage::Pong => "PONG\n".to_string(),

        }

    }

    pub fn decode(input: &str) -> Result<ServerMessage, String> {
        let input = input.trim();

        let mut parts = input.splitn(2, '|');
        let kind = parts.next().ok_or("ServerMessage: Malformed protocol message, missing kind.")?;
        let content = parts.next();
        match kind {
            "LOGIN_OK" => {
                let content = content.ok_or("ServerMessage: Missing LOGIN_OK content.")?;
                Ok(ServerMessage::LoginOk { username: content.to_string() } )
            }

            "LOGIN_ERROR" => {
                let content = content.ok_or("ServerMessage: Missing login content.")?;
                Ok(ServerMessage::LoginErr { error: content.to_string()  })
            }

            "PONG" => {
                Ok(ServerMessage::Pong)
            }
            _ => Err("ServerMessage: Unknown ServerMessage type".into())
        }
    }


}

