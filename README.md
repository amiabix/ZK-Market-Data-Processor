# ZisK Private Value Example

This project demonstrates a standardized approach for handling **public and private inputs** in a cryptographic computation. Public and private values are written in a fixed binary format by `build.rs` and read by the main program. This approach is simple, clear, and easy to extend for small-to-medium projects.

---

## Table of Contents
- [Overview](#overview)
- [Project Structure](#project-structure)
- [Standardized Input Format](#standardized-input-format)
- [How It Works](#how-it-works)
- [Setup & Usage](#setup--usage)
- [Code Walkthrough](#code-walkthrough)
- [Security Notes](#security-notes)

---

## Overview

- **Public input:** Number of hash rounds (`n`)
- **Private input:** A secret value (32 bytes)
- **Goal:** Hash the private value `n` times using SHA-256, and output the final hash (split into 8 public values).
- **Privacy:** The secret is never revealed in outputs or logs.

---

## Project Structure

```
sha_hasher/
├── build.rs              # Writes build/input.bin (public + private)
├── Cargo.toml
├── build/
│   └── input.bin         # Binary input (public + private)
├── src/
│   └── main.rs           # Main program logic
└── ...
```

---

## Standardized Input Format

- **Bytes 0..8:**   Public input (u64, little-endian)
- **Bytes 8..40:**  Private input ([u8; 32])

**Example:**
- `n = 5` (public) → `[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]`
- `secret = [42; 32]` (private) → 32 bytes, all 42

---

## How It Works

1. **Edit `build.rs`** to set your public and private values.
2. **Run `cargo build` or `cargo run`** to generate `build/input.bin`.
3. **Run the main program** (`cargo run`):
   - Reads `build/input.bin`
   - Uses the first 8 bytes as the public input (`n`)
   - Uses the next 32 bytes as the private input (secret)
   - Hashes the secret `n` times
   - Outputs the final hash in 8 public 32-bit chunks

---

## Setup & Usage

### 1. Set your inputs in `build.rs`
```rust
let n: u64 = 5; // public input
let secret: [u8; 32] = [42; 32]; // private input
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
- Writes `build/input.bin` in the standardized format: public input first, then private input.

### main.rs
- Reads `build/input.bin`
- Extracts public and private values by byte offset
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