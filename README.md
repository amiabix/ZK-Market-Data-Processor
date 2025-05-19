# ZisK: Standardized Public and Private Input Handling

This project helps demonstrates the handling of **public and private inputs** in ZisK, In this example Public and private values are written in a fixed binary format by `build.rs` and read by the main program. This approach is simple, auditable, and easy to extend for small-to-medium projects.

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

- **Purpose:** Demonstrate best practices for securely handling public and private inputs in a computation witb ZisK.
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

> **Note:** For ergonomic and reusable parsing, we use a generic helper function to extract fields from the input buffer.

**Summary Table:**
| Bytes         | Meaning         | How to Read in Rust                        |
|---------------|----------------|--------------------------------------------|
| 0..8          | Public input n  | `u64::from_le_bytes(extract_array::<8>(&input, 0))`         |
| 8..40         | Private secret  | `extract_array::<32>(&input, 8)`           |

**Example:**
- `n = 5` (public) → `[0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]`
- `secret = [42; 32]` (private) → 32 bytes, all 42

---

## Reference Example

Suppose you want to set:
- **Public input:** `n = 5`
- **Private input:** 32 bytes, all set to `42`

### How `build.rs` writes the inputs

```rust
let n: u64 = 5; // public input
let secret: [u8; 32] = [42; 32]; // private input

let mut file = File::create("build/input.bin")?;
file.write_all(&n.to_le_bytes())?; // Bytes 0..8: public input
file.write_all(&secret)?;          // Bytes 8..40: private input
```

### How `main.rs` reads the inputs (using a generic helper)

```rust
/// Extracts a fixed-size array from a byte slice at a given offset.
fn extract_array<const N: usize>(input: &[u8], start: usize) -> [u8; N] {
    let mut arr = [0u8; N];
    arr.copy_from_slice(&input[start..start + N]);
    arr
}

let n = u64::from_le_bytes(extract_array::<8>(&input, 0)); // public input
let mut hash = extract_array::<32>(&input, 8);             // private input
```

### Summary Table

| Bytes   | Meaning         | How to Read in Rust                        |
|---------|----------------|--------------------------------------------|
| 0..8    | Public input n  | `u64::from_le_bytes(extract_array::<8>(&input, 0))`         |
| 8..40   | Private secret  | `extract_array::<32>(&input, 8)`           |

---

## How Values Are Written and Referenced

### How `build.rs` Writes to `input.bin`
- `build.rs` writes the public input (`n`) as the first 8 bytes (little-endian u64).
- It then writes the private input (`secret`) as the next 32 bytes.
- The resulting file, `build/input.bin`, always has the same structure:
  - Bytes 0..8: public input
  - Bytes 8..40: private input

### How `main.rs` References Values from `input.bin`
- `main.rs` opens and reads `build/input.bin` into a byte buffer.
- It extracts the public input with:
  ```rust
  let n = u64::from_le_bytes(extract_array::<8>(&input, 0));
  ```
- It extracts the private input with:
  ```rust
  let mut hash = extract_array::<32>(&input, 8);
  ```
- This ensures the program always knows exactly where to find each value, making the code robust and easy to maintain.

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
- Uses the `extract_array` helper to extract public and private values by byte offset
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