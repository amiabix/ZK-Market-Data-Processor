// This example program takes a number `n` as input and computes the SHA-256 hash `n` times sequentially.

use sha2::{Digest, Sha256};
use std::convert::TryInto;
use byteorder::ByteOrder;
use std::fs::{File, metadata};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
struct Public {
    n: u64,
}

#[derive(Serialize, Deserialize)]
struct Private {
    secret: String,
}

#[derive(Serialize, Deserialize)]
struct Input {
    public: Public,
    private: Private,
}

fn prepare_input() -> std::io::Result<()> {
    // Read input.json
    let mut file = File::open("input.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let input: Input = serde_json::from_str(&contents).expect("Invalid JSON");

    // Prepare public and private fields
    let n = input.public.n;
    let mut secret_bytes = input.private.secret.as_bytes().to_vec();
    secret_bytes.resize(32, 0); // pad or truncate to 32 bytes

    // Write input.bin (public + private)
    let mut bin_file = File::create("input.bin")?;
    bin_file.write_all(&n.to_le_bytes())?;
    bin_file.write_all(&secret_bytes)?;

    // Write public.json
    let public_json = serde_json::to_string_pretty(&input.public).unwrap();
    let mut pub_file = File::create("public.json")?;
    pub_file.write_all(public_json.as_bytes())?;

    Ok(())
}

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