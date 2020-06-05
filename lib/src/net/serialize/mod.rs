use bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Data {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub action: String,
}

pub fn serialize(data: Data) -> Vec<u8> {
    bincode::serialize(&data).unwrap()
}

pub fn deserialize(encoded: Vec<u8>) -> Data {
    let decoded: Data = bincode::deserialize(&encoded).unwrap();
    decoded
}
