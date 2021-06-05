mod block;

use bincode;
use block::Block;
use std::option::Option;

fn main() {
    demo_code2();
}

fn demo_code() {
    let mut v = Option::None;
    v = Option::Some(String::from("Hello, world"));
    println!("{}", v.unwrap());

    let mut v = Vec::new();
    for i in 0..10 {
        v.push(i)
    }
    println!("{:?}", v);

    let encoded: Vec<u8> = bincode::serialize(&v).unwrap();
    let decoded: Vec<u32> = bincode::deserialize(&encoded[..]).unwrap();
    println!("{:?}", decoded)
}

fn demo_code2() {
    let mut b = Block::empty();
    b.initialize(10, 10);
    let b2 = Block::from(&b.serialize()[..]);
}
