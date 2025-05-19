#![no_main]
ziskos::entrypoint!(main);

use ziskos::{read_input, set_output};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

fn main() {
    // Read the input data as a byte array from ZisK
    let input: Vec<u8> = read_input();
    let n = u64::from_le_bytes(input[0..8].try_into().unwrap());
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&input[8..40]);

    // Compute SHA-256 hashing 'n' times
    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(hash);
        let digest = &hasher.finalize();
        hash = Into::<[u8; 32]>::into(*digest);
    }

    // Output the final hash in 8 chunks (public output)
    for i in 0..8 {
        let val = u32::from_be_bytes(hash[i * 4..i * 4 + 4].try_into().unwrap());
        set_output(i, val);
    }
} 