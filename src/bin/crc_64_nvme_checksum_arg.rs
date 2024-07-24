/// Generates CRC-64-NVME (aka `CRC-64-Rocksoft`) checksums, using SIMD-accelerated
/// carryless-multiplication, from an input string as a command-line argument.

use std::env;
use crc64fast::Digest;
use std::process::ExitCode;

fn calculate_crc_64_simd(input: &str) -> u64 {
    let mut c = Digest::new();

    c.write(input.as_bytes());

    return c.sum64();
}

fn calculate_crc_64_validate(input: &str) -> u64 {
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

    let crc = crc::Crc::<u64>::new(&CUSTOM_ALG);
    let mut digest = crc.digest();
    digest.update(input.as_bytes());

    return digest.finalize();
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: crc_64_nvm_checksum_file_simd input_string [validate]");
        println!("Optionally including 'validate' in the argument list will skip SIMD calculation for testing.");

        return ExitCode::from(1);
    }

    let input = &args[1];

    if args.len() == 2 {
        println!("{}", calculate_crc_64_simd(input));

        return ExitCode::from(0);
    }

    if args.len() == 3 && "validate" == &args[2] {
        println!("{}", calculate_crc_64_validate(input));

        return ExitCode::from(0);
    }

    println!("An error occurred, likely due to bad command-line arguments.");

    return ExitCode::from(1);
}