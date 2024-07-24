/// Generates CRC-64-NVME (aka `CRC-64-Rocksoft`) checksums, without SIMD-acceleration,
/// from a file on disk. Use for validation and benchmarking.
///
/// https://www.spinics.net/lists/linux-crypto/msg62324.html

use std::env;

fn main() {
    const CUSTOM_ALG: crc::Algorithm<u64> = crc::Algorithm {
        width: 64,
        poly: 0xAD93D23594C93659,
        init: 0xFFFFFFFFFFFFFFFF,
        refin: true,
        refout: true,
        xorout: 0xFFFFFFFFFFFFFFFF,
        check: 0xae8b14860a799888,
        residue: 0x0000000000000000
    };

    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let crc = crc::Crc::<u64>::new(&CUSTOM_ALG);
    let mut digest = crc.digest();
    digest.update(std::fs::read(file).unwrap().as_slice());

    println!("{}", digest.finalize());
}