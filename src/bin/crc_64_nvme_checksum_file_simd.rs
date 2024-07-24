/// Generates CRC-64-NVME (aka `CRC-64-Rocksoft`) checksums, using SIMD-accelerated
/// carryless-multiplication, from a file on disk.

use std::env;
use crc64fast::Digest;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let mut c = Digest::new();

    c.write(std::fs::read(file).unwrap().as_slice());

    println!("{}", c.sum64());
}