use crate::block::Block;

pub struct Bucket {
    pub blocks: Vec<Block>,
    pub z: u8,
}

impl Bucket {
    pub fn empty() -> Bucket {
        Bucket {
            blocks: Vec::new(),
            z: 0,
        }
    }

    pub fn with_z(z: u8) -> Bucket {
        Bucket {
            blocks: Vec::new(),
            z,
        }
    }

    pub fn initialize(&mut self, data_size: u32, gn: u32) {
        for _ in 0..self.z {
            let mut b = Block::empty();
            b.initialize(data_size, gn);
            self.blocks.push(b);
        }
    }
    pub fn reset_blocks(&mut self, data_size: u32, gn: u32) {
        for item in self.blocks.iter_mut() {
            item.reset(data_size, gn);
        }
    }
    pub fn sample_randomness(&mut self) {
        for item in self.blocks.iter_mut() {
            item.generate_r();
        }
    }
    pub fn display_blocks(&mut self) {
        for item in self.blocks.iter() {
            println!("({}, {})", item.id, item.tree_label);
        }
        println!("")
    }
    pub fn aes_encrypt_blocks(&mut self, data_size: u32, aes_key: &[u8]) {}
    pub fn aes_decrypt_blocks(&mut self, data_size: u32, aes_key: &[u8]) {}
    pub fn serialize(&mut self, data_size: u32) -> Vec<u8> {
        Vec::new()
    }
    pub fn serialize_to_buffer(&mut self, buf: Vec<u8>) {}
}
