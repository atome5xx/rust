use std::net::UdpSocket;
use std::time::Duration;

use crate::dns::{build_query, read_name};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

pub fn run_client() {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    let server_addr = "127.0.0.1:5353";
    let domain = "example.local";
    let request = build_query(domain);

    socket.send_to(&request, server_addr).unwrap();

    let mut buf = [0u8; 512];
    match socket.recv_from(&mut buf) {
        Ok((size, _)) => {
            let mut cursor = Cursor::new(&buf[..size]);
            cursor.set_position(6);
            let qdcount = cursor.read_u16::<BigEndian>().unwrap();
            let ancount = cursor.read_u16::<BigEndian>().unwrap();

            cursor.set_position(12);
            let _ = read_name(&mut cursor).unwrap();
            cursor.set_position(cursor.position() + 4);

            if ancount > 0 {
                let _ = read_name(&mut cursor).unwrap();
                let _rtype = cursor.read_u16::<BigEndian>().unwrap();
                let _rclass = cursor.read_u16::<BigEndian>().unwrap();
                let _ttl = cursor.read_u32::<BigEndian>().unwrap();
                let _rdlength = cursor.read_u16::<BigEndian>().unwrap();
                let ip_bytes = [
                    cursor.read_u8().unwrap(),
                    cursor.read_u8().unwrap(),
                    cursor.read_u8().unwrap(),
                    cursor.read_u8().unwrap(),
                ];
                println!("Réponse IP pour {}: {}", domain, ip_bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("."));
            } else {
                println!("Aucune réponse.");
            }
        }
        Err(e) => println!("Erreur de réception: {}", e),
    }
}
