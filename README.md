crc64fast-nvme
=========

[![Build status](https://github.com/awesomized/crc64fast-nvme/workflows/Rust/badge.svg)](https://github.com/awesomized/crc64fast-nvme/actions?query=workflow%3ARust)

SIMD-accelerated carryless-multiplication [CRC-64/NVME](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-nvme) checksum computation
(similar to [crc32fast](https://crates.io/crates/crc32fast) and forked from [crc64fast](https://github.com/tikv/crc64fast) which calculates [CRC-64/XZ](https://reveng.sourceforge.io/crc-catalogue/all.htm#crc.cat.crc-64-xz) [a.k.a `CRC-64/GO-ECMA`]).

`CRC-64/NVME` comes from the [NVM Express® NVM Command Set Specification](https://nvmexpress.org/wp-content/uploads/NVM-Express-NVM-Command-Set-Specification-1.0d-2023.12.28-Ratified.pdf) (Revision 1.0d, December 2023) and has also been implemented in the [Linux kernel](https://github.com/torvalds/linux/blob/786c8248dbd33a5a7a07f7c6e55a7bfc68d2ca48/lib/crc64.c#L66-L73) (where it's called `CRC-64/Rocksoft`). Note that the Check value in the spec is incorrect (Section 5.2.1.3.4, Figure 120, page 83).

SIMD-accelerated carryless-multiplication is based on the Intel [Fast CRC Computation for Generic Polynomials Using PCLMULQDQ Instruction](https://web.archive.org/web/20131224125630/https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/fast-crc-computation-generic-polynomials-pclmulqdq-paper.pdf) paper.

## Changes from [crc64fast](https://github.com/tikv/crc64fast)

Primarily changes the `CRC-64/XZ` (aka `CRC-64/GO-ECMA`) polynomial from [crc64fast](https://github.com/tikv/crc64fast) (which uses the `ECMA-182` polynomial [`0x42F0E1EBA9EA3693`]) to use the `NVME` polynomial (`0xAD93D23594C93659`), plus re-calculates the input parameters (tables, keys, mu, and reciprocal polynomial) for carryless-multiplication.

## Usage

```rust
use crc64fast_nvme::Digest;

let mut c = Digest::new();
c.write(b"hello ");
c.write(b"world!");
let checksum = c.sum64();
assert_eq!(checksum, 0xd9160d1fa8e418e3);
```

## CLI example
A simple CLI implementation can be found in [crc_64_nvme_checksum.rs](src\bin\crc_64_nvme_checksum.rs), which will calculate the CRC-64/NVME checksum for a file on disk.

## Other CRC-64 implementations
Tooling to re-calculate input parameters for other CRC-64 implementations/polynomials is supplied in [src\bin](src\bin).

## Performance

`crc64fast` provides two fast implementations, and the most performance one will
be chosen based on CPU feature at runtime.

* a fast, platform-agnostic table-based implementation, processing 16 bytes at a time.
* a SIMD-carryless-multiplication based implementation on modern processors:
    * using PCLMULQDQ + SSE 4.1 on x86/x86_64
    * using PMULL + NEON on AArch64 (64-bit ARM)

| Algorithm         | Throughput (x86_64) | Throughput (aarch64) |
|:------------------|--------------------:|---------------------:|
| [crc 3.0.1]       |  0.5 GiB/s          |  0.3 GiB/s           |
| crc64fast (table) |  2.3 GiB/s          |  1.8 GiB/s           |
| crc64fast (simd)  | 28.2 GiB/s          | 20.0 GiB/s           |

[crc 3.0.1]: https://docs.rs/crc/3.0.1/crc/index.html

## TODO

This crate is mainly intended for use in TiKV only.
Features beyond AArch64 are unlikely to be implemented.

* [x] AArch64 support based on PMULL
* [ ] `no_std` support
* [x] Fuzz test
* [ ] Custom polynomial

## License

crc64fast is dual-licensed under

* Apache 2.0 license ([LICENSE-Apache](./LICENSE-Apache) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
