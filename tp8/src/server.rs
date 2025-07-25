use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::protocol::Packet;

pub fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(0) => false,
        Ok(size) => {
            let packet = Packet::deserialize(&buffer[..size]);
            println!("Received: {:?}", packet);
            true
        }
        Err(_) => {
            eprintln!("Client disconnected");
            false
        }
    } {}
}