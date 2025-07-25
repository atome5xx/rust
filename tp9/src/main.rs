use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(&addr).await.expect("Erreur de binding");

    println!("Serveur WebSocket en écoute sur ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    println!("Client connecté");

                    let (mut write, mut read) = ws_stream.split();

                    while let Some(message_result) = read.next().await {
                        match message_result {
                            Ok(msg) => {
                                println!("Message reçu: {}", msg);

                                if msg.is_text() || msg.is_binary() {
                                    write.send(msg).await.expect("Erreur d'envoi");
                                }
                            }
                            Err(e) => {
                                eprintln!("Erreur de lecture: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Handshake échoué: {}", e),
            }
        });
    }
}
