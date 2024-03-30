use async_tungstenite::tungstenite::Result;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088").await?;
    println!("WebSocket server started on ws://127.0.0.1:8088");

    let (tx, _rx) = broadcast::channel(2);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(handle_connection(stream, tx));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, tx: broadcast::Sender<String>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut ws_sender,mut ws_receiver) = ws_stream.split();
    
    ws_sender.send(Message::from("Sending special message")).await.unwrap(); // Send message only to connected client

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Some(message) = ws_receiver.next().await {
            if let Ok(Message::Text(text)) = message {
                tx_clone.send(text).unwrap(); // send message to everyone connected
            }
        }
    });

    let mut rx = tx.subscribe();

    while let Ok(message) = rx.recv().await {
        if let Err(e) = ws_sender.send(Message::Text(message)).await {
            eprintln!("Error sending message to WebSocket client: {:?}", e);
            break;
        }
    }
}
