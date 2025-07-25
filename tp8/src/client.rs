use std::net::TcpStream;
use std::io::{self, Write, Read};
use crate::protocol::{Packet, OpCode};

pub fn run_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to server.");

    loop {
        let mut input = String::new();
        print!("Enter message (or 'exit'): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed == "exit" {
            let packet = Packet { opcode: OpCode::Logout, payload: String::from("Bye!") };
            stream.write_all(&packet.serialize())?;
            break;
        }

        let packet = Packet {
            opcode: OpCode::Message,
            payload: trimmed.to_string(),
        };

        stream.write_all(&packet.serialize())?;
    }
    Ok(())
}
