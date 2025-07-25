use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use futures::{SinkExt, StreamExt};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:9001").unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Connexion échouée");
    println!("Connecté au serveur WebSocket");

    let (mut write, mut read) = ws_stream.split();

    // Envoie d'un message texte
    let msg = "Hello depuis le client!";
    write.send(Message::Text(msg.into())).await.unwrap();
    println!("Message envoyé: {}", msg);

    // Réception de la réponse (echo)
    if let Some(Ok(response)) = read.next().await {
        println!("Réponse reçue: {}", response);
    }
}
