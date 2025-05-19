use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

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

fn main() -> std::io::Result<()> {
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