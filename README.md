# ZisK: Public and Private Input Handling

**TL;DR:** This project shows how to handle **public and private inputs** in ZisK. In this setup we use build.rs to work with both public and private values in a simple binary format, which the main program then reads and works with.

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

---

## Project Structure

```
sha_hasher/
├── build.rs              # Writes public.bin (public) and private.bin (private)
├── Cargo.toml
├── public.bin            # Public input (e.g., number of hash rounds)
├── private.bin           # Private input (e.g., secret value)
├── src/
│   └── main.rs           # Main program logic
└── ...
```

---

## Standardized Input Format

- **public.bin:** 8 bytes, little-endian u64 (public input)
- **private.bin:** 32 bytes ([u8; 32], private input)

**Summary Table:**
| File         | Bytes         | Meaning         | How to Read in Rust                        |
|--------------|--------------|----------------|--------------------------------------------|
| public.bin   | 0..8          | Public input n  | `u64::from_le_bytes(pub_input)`            |
| private.bin  | 0..32         | Private secret  | `private_input` as `[u8; 32]`              |

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

### How `main.rs` reads the inputs

```rust
let mut pub_input = [0u8; 8];
File::open("public.bin").expect("public.bin not found").read_exact(&mut pub_input).expect("Failed to read public.bin");
let n = u64::from_le_bytes(pub_input);

let mut hash = [0u8; 32];
File::open("private.bin").expect("private.bin not found").read_exact(&mut hash).expect("Failed to read private.bin");
```

---

## How Values Are Written and Referenced

### How `build.rs` Writes to `public.bin` and `private.bin`
- `build.rs` writes the public input (`n`) as 8 bytes (little-endian u64) to `public.bin`.
- It writes the private input (`secret`) as 32 bytes to `private.bin`.

### How `main.rs` References Values
- `main.rs` reads 8 bytes from `public.bin` and interprets them as `n`.
- It reads 32 bytes from `private.bin` and uses them as the secret value.

---

## How It Works

1. **Edit `build.rs`** to set your public and private values.
2. **Run `cargo build` or `cargo run`** to generate `public.bin` and `private.bin`.
3. **Run the main program** (`cargo run`):
   - Reads `public.bin` and `private.bin`
   - Uses the public input (`n`) and private input (secret)
   - Hashes the secret `n` times
   - Outputs the final hash in 8 public 32-bit chunks

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

### 3. Run the main program
```sh
cargo run
```

---

## Code Walkthrough

### build.rs
- Writes `public.bin` (public input) and `private.bin` (private input) in a clear, separated format.

### main.rs
- Reads `public.bin` and `private.bin`
- Uses the public input (`n`) and private input (secret)
- Hashes the private value `n` times
- Outputs the final hash in 8 public 32-bit chunks

---

## Security Notes
- **Never** print or log the private input.
- Only the public input and the final hash are output.
- This approach is simple and effective for most small-to-medium projects.

---

## Example Output
```
public 0: 0x...
public 1: 0x...
...
public 7: 0x...
```

---