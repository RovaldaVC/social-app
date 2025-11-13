use tokio_tungstenite::connect_async;
use futures::{StreamExt, SinkExt};
use serde::{Serialize, Deserialize};
use chrono::Local;
use tokio::io::{self, AsyncBufReadExt};
use std::env;

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

#[tokio::main]
async fn main() {
    // Get username from command line args
    let username = env::args().nth(1).expect("Provide a username as first argument");

    // Connect to WebSocket server
    let url = url::Url::parse("ws://127.0.0.1:8082").unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected as '{}'", username);

    let (mut write, mut read) = ws_stream.split();

    // Task to read messages from server
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let tokio_tungstenite::tungstenite::Message::Text(txt) = msg {
                if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&txt) {
                    if chat_msg.username == username && chat_msg.message_type == MessageType::UserMessage {
                        println!("You [{}]: {}", chat_msg.timestamp, chat_msg.content);
                    } else if chat_msg.message_type == MessageType::UserMessage {
                        println!("{} [{}]: {}", chat_msg.username, chat_msg.timestamp, chat_msg.content);
                    } else {
                        println!("* {} {}", chat_msg.username, chat_msg.content);
                    }
                }
            }
        }
    });

    // Task to read input from terminal and send messages
    let write_task = tokio::spawn(async move {
        let stdin = io::BufReader::new(io::stdin());
        let mut lines = stdin.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            let msg = ChatMessage {
                username: username.clone(),
                content: line.trim().to_string(),
                timestamp: Local::now().format("%H:%M:%S").to_string(),
                message_type: MessageType::UserMessage,
            };

            let json = serde_json::to_string(&msg).unwrap();
            let _ = write.send(tokio_tungstenite::tungstenite::Message::Text(json)).await;
        }
    });

    // Wait for both tasks
    let _ = tokio::join!(read_task, write_task);
}
