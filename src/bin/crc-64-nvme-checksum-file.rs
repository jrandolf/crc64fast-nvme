use std::env;
use crc64fast::Digest;

// Generates CRC-64-NVME (aka `CRC-64-Rocksoft`) Checksums, using SIMD, from a file on disk
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let mut c = Digest::new();

    c.write(std::fs::read(file).unwrap().as_slice());

    println!("{}", c.sum64());
}