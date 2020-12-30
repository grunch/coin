use std::fmt::{ self, Display, Formatter };
use easy_hasher::easy_hasher::*;

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub payload: String,
    pub hash: String,
    pub prev_hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        payload: String,
        prev_hash: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            payload,
            prev_hash,
            hash: String::from(""),
        }
    }

    pub fn calculate_hash(&self) -> String {
        let s = format!(
            "{}{}{}{}",
            &self.index,
            &self.timestamp,
            &self.payload,
            &self.prev_hash,
        );
        let hash = sha256(&s);

        hash.to_hex_string()
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block idx: {} hash: {} prevHash: {} Payload: {}",
            &self.index,
            &self.hash,
            &self.prev_hash,
            &self.payload,
        )
    }
}
