use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Result};
use std::net::Ipv4Addr;

pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
}

pub struct DnsQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

pub struct DnsAnswer {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub rdata: Ipv4Addr,
}

pub fn write_name(buf: &mut Vec<u8>, name: &str) {
    for part in name.split('.') {
        buf.push(part.len() as u8);
        buf.extend_from_slice(part.as_bytes());
    }
    buf.push(0); // end
}

pub fn read_name(cursor: &mut Cursor<&[u8]>) -> Result<String> {
    let mut name = String::new();
    loop {
        let len = cursor.read_u8()?;
        if len == 0 {
            break;
        }
        let mut label = vec![0; len as usize];
        cursor.read_exact(&mut label)?;
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(&String::from_utf8_lossy(&label));
    }
    Ok(name)
}

pub fn build_query(domain: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.write_u16::<BigEndian>(0x1234).unwrap(); // ID
    buf.write_u16::<BigEndian>(0x0100).unwrap(); // standard query
    buf.write_u16::<BigEndian>(1).unwrap(); // QDCOUNT
    buf.write_u16::<BigEndian>(0).unwrap(); // ANCOUNT
    buf.write_u16::<BigEndian>(0).unwrap(); // NSCOUNT
    buf.write_u16::<BigEndian>(0).unwrap(); // ARCOUNT

    write_name(&mut buf, domain);
    buf.write_u16::<BigEndian>(1).unwrap(); // QTYPE A
    buf.write_u16::<BigEndian>(1).unwrap(); // QCLASS IN

    buf
}

pub fn parse_query(buf: &[u8]) -> Result<(DnsHeader, DnsQuestion)> {
    let mut cursor = Cursor::new(buf);
    let header = DnsHeader {
        id: cursor.read_u16::<BigEndian>()?,
        flags: cursor.read_u16::<BigEndian>()?,
        qdcount: cursor.read_u16::<BigEndian>()?,
        ancount: cursor.read_u16::<BigEndian>()?,
    };
    cursor.set_position(12);
    let qname = read_name(&mut cursor)?;
    let qtype = cursor.read_u16::<BigEndian>()?;
    let qclass = cursor.read_u16::<BigEndian>()?;
    Ok((header, DnsQuestion { qname, qtype, qclass }))
}

pub fn build_response(header: &DnsHeader, question: &DnsQuestion, ip: Ipv4Addr) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.write_u16::<BigEndian>(header.id).unwrap();
    buf.write_u16::<BigEndian>(0x8180).unwrap(); // standard response
    buf.write_u16::<BigEndian>(1).unwrap(); // QDCOUNT
    buf.write_u16::<BigEndian>(1).unwrap(); // ANCOUNT
    buf.write_u16::<BigEndian>(0).unwrap(); // NSCOUNT
    buf.write_u16::<BigEndian>(0).unwrap(); // ARCOUNT

    write_name(&mut buf, &question.qname);
    buf.write_u16::<BigEndian>(question.qtype).unwrap();
    buf.write_u16::<BigEndian>(question.qclass).unwrap();

    // Answer
    write_name(&mut buf, &question.qname);
    buf.write_u16::<BigEndian>(1).unwrap(); // A
    buf.write_u16::<BigEndian>(1).unwrap(); // IN
    buf.write_u32::<BigEndian>(60).unwrap(); // TTL
    buf.write_u16::<BigEndian>(4).unwrap(); // RDLENGTH
    buf.extend_from_slice(&ip.octets());

    buf
}
