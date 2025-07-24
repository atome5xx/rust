use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    fs::OpenOptions,
    sync::Mutex,
};
use tokio::fs;
use chrono::Local;
use std::{sync::Arc, path::Path};

const LOG_FILE: &str = "logs/server.log";

async fn init_log_file(path: &str) -> anyhow::Result<Arc<Mutex<tokio::fs::File>>> {
    let dir = Path::new(path).parent().unwrap();

    if !dir.exists() {
        fs::create_dir_all(dir).await?;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;

    Ok(Arc::new(Mutex::new(file)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let log_file = init_log_file(LOG_FILE).await?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serveur en écoute sur 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Connexion reçue de {}", addr);

        let log_file = log_file.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, log_file, addr).await {
                eprintln!("Erreur client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, log_file: Arc<Mutex<tokio::fs::File>>, client_addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
        let log_entry = format!("{} [{}] {}\n", timestamp, client_addr, line);

        let mut file = log_file.lock().await;
        file.write_all(log_entry.as_bytes()).await?;
    }

    Ok(())
}
