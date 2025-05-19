use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// Standardized input.bin format:
// Bytes 0..8:   Public input (u64, little-endian)
// Bytes 8..40:  Private input ([u8; 32])
const OUTPUT_DIR: &str = "build/";
const FILE_NAME: &str = "input.bin";

fn main() -> io::Result<()> {
    let n: u64 = 5; // public input
    let secret: [u8; 32] = [42; 32]; // private input (all bytes are 42)

    // Ensure the output directory exists
    let output_dir = Path::new(OUTPUT_DIR);
    if !output_dir.exists() {
        // Create the directory and any necessary parent directories
        fs::create_dir_all(output_dir)?; 
    }

    // Create the file and write the 'n' value in little-endian format
    let file_path = output_dir.join(FILE_NAME);
    let mut file = File::create(&file_path)?;
    file.write_all(&n.to_le_bytes())?;  // write public input
    file.write_all(&secret)?;          // write private input

    Ok(())
}
