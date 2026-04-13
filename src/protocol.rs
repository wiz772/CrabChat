pub enum Message {

    Chat { user: String, content: String},
    Login { user: String },
    Ping,

}

