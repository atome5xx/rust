use std::collections::HashMap;
use std::net::UdpSocket;

use crate::dns::{build_response, parse_query};

pub fn run_server() {
    let socket = UdpSocket::bind("127.0.0.1:5353").expect("Bind failed");

    let mut records = HashMap::new();
    records.insert("example.local".to_string(), "192.168.0.1".parse().unwrap());

    println!("Serveur DNS en écoute sur 127.0.0.1:5353");

    let mut buf = [0u8; 512];
    loop {
        if let Ok((size, src)) = socket.recv_from(&mut buf) {
            if let Ok((header, question)) = parse_query(&buf[..size]) {
                println!("Requête pour: {}", question.qname);
                if let Some(ip) = records.get(&question.qname) {
                    let response = build_response(&header, &question, *ip);
                    socket.send_to(&response, src).unwrap();
                } else {
                    println!("Nom non trouvé : {}", question.qname);
                }
            }
        }
    }
}
