use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tungstenite::{connect, Message, WebSocket};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessage {
    method: String,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectMessage {
    method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinMessage {
    method: String,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStateMessage {
    method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetDirectionMessage {
    method: String,
    direction: String,
    index: usize,
}

pub struct WebSocketClient {
    socket: Arc<Mutex<WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>>>,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        let (socket, response) = connect("ws://localhost:3012/socket").expect("Can't connect");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }
        WebSocketClient {
            socket: Arc::new(Mutex::new(socket)),
        }
    }

    pub fn send_message(&self, string: &str) -> String {
        let mut socket = self.socket.lock().unwrap();
        socket.send(Message::Text(string.into())).unwrap();
        let msg = socket.read_message().expect("Error reading message");
        let msg = msg.to_string();
        println!("Received: {}", msg);
        msg
    }

    pub fn connect_to_server(&self) -> String {
        let connect_message = ConnectMessage {
            method: "Connect".to_string(),
        };
        let serialized_message = serde_json::to_string(&connect_message).unwrap();
        self.send_message(&serialized_message)
    }

    pub fn create_game(&self, client: String) -> String {
        let create_message = CreateMessage {
            method: "Create".to_string(),
            user_id: client,
        };
        let serialized_message = serde_json::to_string(&create_message).unwrap();
        self.send_message(&serialized_message)
    }

    pub fn join_game(&self, user_id: &String) -> String {
        let join_message = JoinMessage {
            method: "Join".to_string(),
            user_id: user_id.to_string(),
        };
        let serialized_message = serde_json::to_string(&join_message).unwrap();
        self.send_message(&serialized_message)
    }

    pub fn get_game_state(&self) -> String {
        let get_message = GetStateMessage {
            method: "Get".to_string(),
        };
        let serialized_message = serde_json::to_string(&get_message).unwrap();
        self.send_message(&serialized_message)
    }

    pub fn set_direction(&self, direction: String, index: usize) {
        let set_direction_message = SetDirectionMessage {
            method: "Set".to_string(),
            direction,
            index,
        };
        let serialized_message = serde_json::to_string(&set_direction_message).unwrap();
        self.send_message(&serialized_message);
    }
}
