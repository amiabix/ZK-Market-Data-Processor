use std::fs::File;
use std::io::{self, Write};


fn main() -> io::Result<()> {
    let n: u64 = 5; // public input
    let secret: [u8; 32] = [255; 32]; // private input

    // Write public input
    let mut pub_file = File::create("public.bin")?;
    pub_file.write_all(&n.to_le_bytes())?;

    // Write private input
    let mut priv_file = File::create("private.bin")?;
    priv_file.write_all(&secret)?;

    Ok(())
}
