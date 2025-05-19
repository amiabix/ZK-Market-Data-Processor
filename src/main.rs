use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use byteorder::ByteOrder;

fn main() {
    // Read public input
    let mut pub_input = [0u8; 8];
    File::open("public.bin").expect("public.bin not found").read_exact(&mut pub_input).expect("Failed to read public.bin");
    let n = u64::from_le_bytes(pub_input);

    // Read private input
    let mut hash = [0u8; 32];
    File::open("private.bin").expect("private.bin not found").read_exact(&mut hash).expect("Failed to read private.bin");

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