use bincode;
use serde::{Deserialize, Serialize};

const NONCE_LENGTH: u32 = 16;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    pub data: Vec<u8>,
    pub id: u32,
    pub tree_label: u32,
    pub r: Vec<u8>,
}

impl Block {
    pub fn empty() -> Block {
        Block {
            data: Vec::new(),
            r: Vec::new(),
            tree_label: 0,
            id: 0,
        }
    }

    pub fn with_id(gn: u32) -> Block {
        let mut b = Block::empty();
        b.id = gn;
        return b;
    }

    pub fn deserialize(b: &[u8]) -> Block {
        let b: Block = bincode::deserialize(b).unwrap();
        return b;
    }

    pub fn serialize(&self) -> Vec<u8> {
        let encoded = bincode::serialize(self).unwrap();
        return encoded;
    }

    pub fn initialize(&mut self, data_size: u32, gn: u32) {
        self.tree_label = 0;
        self.id = gn;
        self.generate_data(data_size);
        self.generate_r();
    }

    fn generate_data(&mut self, fdata_size: u32) {
        for i in 0..fdata_size {
            self.data.push((i as u8 % 26) + 64);
        }
    }

    fn generate_r(&mut self) {
        for i in 0..NONCE_LENGTH {
            self.r.push(i as u8)
        }
    }

    fn is_dummy(&self, gn: u32) -> bool {
        self.id == gn
    }
}
