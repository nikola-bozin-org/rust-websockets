use async_tungstenite::tungstenite::Result;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::protocol::Message;
use async_tungstenite::tungstenite::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (mut ws_stream, _) = tokio_tungstenite::connect_async("ws://127.0.0.1:8088")
        .await
        .expect("Error during WebSocket handshake");

    ws_stream.send(Message::Text("Hello, server!".into())).await.expect("Failed to send message");

    while let Some(Ok(message)) = ws_stream.next().await {
        match message {
            Message::Text(text) => {
                println!("Received message: {}", text);
            },
            _ => {
                eprintln!("Received unsupported message type");
            }
        }
    }

    Ok(())
}