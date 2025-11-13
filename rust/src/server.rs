use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};
use chrono::Local;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    UserMessage,
    SystemNotification,
}

// Struct to decode JWT payload
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SECRET_KEY: &str = "1224.my_secret@key#"; // same as Python

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    let (tx, _) = broadcast::channel::<String>(100);

    println!("WebSocket server running on ws://127.0.0.1:8082");

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(handle_connection(stream, tx, rx));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, tx: broadcast::Sender<String>, mut rx: broadcast::Receiver<String>) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // First message should contain JWT token and username
    let first_msg = ws_receiver.next().await;
    if first_msg.is_none() { return; }

    let token_msg = first_msg.unwrap().unwrap();
    let token_text = if let tokio_tungstenite::tungstenite::Message::Text(txt) = token_msg {
        txt
    } else { return; };

    let token_json: Value = serde_json::from_str(&token_text).unwrap();
    let token_str = token_json["token"].as_str().unwrap();
    let username = token_json["username"].as_str().unwrap().to_string();

    // Verify JWT
    let token_data: Result<TokenData<Claims>, _> = decode::<Claims>(
        token_str,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    if token_data.is_err() {
        let _ = ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(
            "{\"error\":\"Invalid token\"}".to_string()
        )).await;
        return;
    }

    // Spawn a task to forward broadcast messages to this client
    let mut ws_sender_clone = ws_sender.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let _ = ws_sender_clone.send(tokio_tungstenite::tungstenite::Message::Text(msg)).await;
        }
    });

    // Send "joined" system notification
    let join_msg = ChatMessage {
        username: username.clone(),
        content: "joined the chat.".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };
    let _ = tx.send(serde_json::to_string(&join_msg).unwrap());

    // Handle messages from this client
    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
            if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&txt) {
                let json = serde_json::to_string(&chat_msg).unwrap();
                let _ = tx.send(json);
            }
        }
    }

    // Send "left" system notification
    let leave_msg = ChatMessage {
        username: username.clone(),
        content: "left the chat.".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };
    let _ = tx.send(serde_json::to_string(&leave_msg).unwrap());
}