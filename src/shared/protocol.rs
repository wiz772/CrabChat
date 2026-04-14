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

    // pub fn decode(input: &str) -> Result<Message, String> {
    //     let mut parts = input.splitn(2, '|');
    //     let kind =
    //     let content = 

    // }
}

