use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum OpCode {
    Login,
    Message,
    Logout,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub opcode: OpCode,
    pub payload: String,
}

impl Packet {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Packet {
        bincode::deserialize(bytes).unwrap()
    }
}
