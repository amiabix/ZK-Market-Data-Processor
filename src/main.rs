// This example program takes a number `n` as input and computes the SHA-256 hash `n` times sequentially.

use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use byteorder::ByteOrder;

fn main() {
    // Standardized input.bin format:
    // Bytes 0..8:   Public input (u64, little-endian)
    // Bytes 8..40:  Private input ([u8; 32])
    let mut input = Vec::new();
    File::open("build/input.bin").expect("input.bin not found").read_to_end(&mut input).expect("Failed to read input.bin");

    let n = u64::from_le_bytes(input[0..8].try_into().unwrap()); // public input
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&input[8..40]); // private input

    // Compute SHA-256 hashing 'n' times
    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(hash);
        let digest = &hasher.finalize();
        hash = Into::<[u8; 32]>::into(*digest);
    }

    // Output the final hash in 8 chunks (public output)
    for i in 0..8 {
        let val = byteorder::BigEndian::read_u32(&hash[i * 4..i * 4 + 4]);
        println!("public {}: 0x{:08x}", i, val);
    }
}