use std::fs::{File, create_dir_all};
use std::io::{Read, Write, Result};

fn main() -> Result<()> {
    // Ensure the Bin and build directories exist
    create_dir_all("Bin")?;
    create_dir_all("build")?;

    let n: u64 = 5; // public input
    let secret: [u8; 32] = [255; 32]; // private input

    let mut pub_file = File::create("Bin/public.bin")?;
    pub_file.write_all(&n.to_le_bytes())?;

    let mut priv_file = File::create("Bin/private.bin")?;
    priv_file.write_all(&secret)?;

    // Concatenate Bin/public.bin and Bin/private.bin into build/input.bin
    let mut public = Vec::new();
    let mut private = Vec::new();
    File::open("Bin/public.bin")?.read_to_end(&mut public)?;
    File::open("Bin/private.bin")?.read_to_end(&mut private)?;

    let mut input_file = File::create("build/input.bin")?;
    input_file.write_all(&public)?;
    input_file.write_all(&private)?;

    Ok(())
}
