pub enum Message {

    Chat { content: String },
    Login { user: String },
    Ping,

}

impl Message {

    pub fn encode(&self) -> String {

        match self {

            Message::Chat { content } => format!("CHAT|{}", content),
            Message::Login { user } => format!("LOGIN|{}", user),
            Message::Ping {} => "PING".to_string()

        }

    }

    pub fn decode(input: &str) -> Result<Message, String> {
        let mut parts = input.splitn(2, '|');
        let kind = parts.next().ok_or("Malformed protocol message, missing kind.")?;
        let content = parts.next();
        match kind {
            "CHAT" => {
                let content = content.ok_or("Missing chat content.")?;
                Ok(Message::Chat { content: content.to_string() })
            }

            "LOGIN" => {
                let content = content.ok_or("Missing login content.")?;
                Ok(Message::Login { user: content.to_string() })
            }

            "PING" => {
                Ok(Message::Ping)
            }
            _ => Err("Malformed protocol message, dropping it".into())
        }
    }
}

