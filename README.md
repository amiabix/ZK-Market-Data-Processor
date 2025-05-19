# ZisK: Public and Private Input Handling

**TL;DR:** This project shows an example on how to handle **public and private inputs** in ZisK. In this setup we use build.rs to work with both public and private values in a simple binary format, which the main program then reads and works with using the ZisK API.

---

## Table of Contents
- [Overview](#overview)
- [Project Structure](#project-structure)
- [Standardized Input Format](#standardized-input-format)
- [Reference Example](#reference-example)
- [How Values Are Written and Referenced](#how-values-are-written-and-referenced)
- [How It Works](#how-it-works)
- [Setup & Usage](#setup--usage)
- [Code Walkthrough](#code-walkthrough)
- [Security Notes](#security-notes)
- [Example Output](#example-output)

---

## Overview

- **Purpose:** Demonstrate best practices for handling public and private inputs with ZisK.
- **Public input:** Number of hash rounds (`n`)
- **Private input:** A secret value (32 bytes)
- **Goal:** Hash the private value `n` times using SHA-256, and output the final hash (split into 8 public values).
- **Privacy:** The secret is never revealed in outputs or logs.
- **ZisK compatibility:** The main program uses `#![no_main]`, `ziskos::entrypoint!(main)`, `read_input()`, and `set_output()` as required by ZisK for provable programs.

---

## Project Structure

```
sha_hasher/
├── build.rs              # Writes public.bin (public) and private.bin (private)
├── Cargo.toml
├── public.bin            # Public input (e.g., number of hash rounds)
├── private.bin           # Private input (e.g., secret value)
├── src/
│   └── main.rs           # Main program logic (ZisK-compliant)
└── ...
```

---

## Standardized Input Format

- **public.bin:** 8 bytes, little-endian u64 (public input)
- **private.bin:** 32 bytes ([u8; 32], private input)
- **ZisK input buffer:** The program expects a single buffer with public input first, then private input (e.g., 8 bytes + 32 bytes = 40 bytes total).

**Summary Table:**
| File         | Bytes         | Meaning         | How to Read in Rust                        |
|--------------|--------------|----------------|--------------------------------------------|
| public.bin   | 0..8          | Public input n  | `u64::from_le_bytes(input[0..8])`          |
| private.bin  | 0..32         | Private secret  | `input[8..40]` as `[u8; 32]`               |

**Example:**
- `n = 5` (public) → `[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]`
- `secret = [255; 32]` (private) → 32 bytes, all 255

---

## Reference Example

Suppose you want to set:
- **Public input:** `n = 5`
- **Private input:** 32 bytes, all set to `255`

### How `build.rs` writes the inputs

```rust
let n: u64 = 5; // public input
let secret: [u8; 32] = [255; 32]; // private input

let mut pub_file = File::create("public.bin")?;
pub_file.write_all(&n.to_le_bytes())?;

let mut priv_file = File::create("private.bin")?;
priv_file.write_all(&secret)?;
```

### How `main.rs` reads the inputs (ZisK-compliant)

```rust
#![no_main]
ziskos::entrypoint!(main);

use ziskos::{read_input, set_output};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

fn main() {
    let input: Vec<u8> = read_input();
    let n = u64::from_le_bytes(input[0..8].try_into().unwrap());
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&input[8..40]);
    // ... hashing and set_output as in code ...
}
```

---

## How Values Are Written and Referenced

### How `build.rs` Writes to `public.bin` and `private.bin`
- `build.rs` writes the public input (`n`) as 8 bytes (little-endian u64) to `public.bin`.
- It writes the private input (`secret`) as 32 bytes to `private.bin`.
- **For ZisK:** The input buffer provided to the program should be the concatenation of these two files (public first, then private).

### How `main.rs` References Values
- `main.rs` uses `read_input()` to get the input buffer.
- It reads 8 bytes for the public input and 32 bytes for the private input from the buffer.
- It uses `set_output()` to set the public outputs.

---

## How It Works

1. **Edit `build.rs`** to set your public and private values.
2. **Run `cargo build` or `cargo run`** to generate `public.bin` and `private.bin`.
3. **Concatenate** the two files to create the ZisK input buffer (e.g., `cat public.bin private.bin > input.bin`).
4. **Run the main program** (in the ZisK environment):
   - Uses `read_input()` to read the buffer
   - Uses the public input (`n`) and private input (secret)
   - Hashes the secret `n` times
   - Uses `set_output()` to set the final hash in 8 public 32-bit chunks

---

## Setup & Usage

### 1. Set your inputs in `build.rs`
```rust
let n: u64 = 5; // public input
let secret: [u8; 32] = [255; 32]; // private input
```

### 2. Build the project
```sh
cargo build
```

### 3. Concatenate the input files for ZisK
```sh
cat public.bin private.bin > input.bin
```

### 4. Run the main program in the ZisK environment
- The ZisK runner will provide `input.bin` as the input buffer to your program.

---

## Code Walkthrough

### build.rs
- Writes `public.bin` (public input) and `private.bin` (private input) in a clear, separated format.

### main.rs
- Uses ZisK API: `#![no_main]`, `ziskos::entrypoint!(main)`, `read_input()`, and `set_output()`
- Reads the input buffer (public + private)
- Uses the public input (`n`) and private input (secret)
- Hashes the private value `n` times
- Sets the final hash in 8 public 32-bit chunks using `set_output()`

---

## Security Notes
- **Never** print or log the private input.
- Only the public input and the final hash are output (via `set_output`).
- This approach is simple and effective for most small-to-medium projects.
- **Note:** For ZisK, all public outputs must be set using `set_output()`.

---

## Example Output
```
public 0: 0x...
public 1: 0x...
...
public 7: 0x...
```

---