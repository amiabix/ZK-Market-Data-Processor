# ZK Market Data Processor - Simple Zero-Knowledge Proof Example

A minimal zero-knowledge proof example that processes market data and generates cryptographic proofs while keeping the processing logic private.

## Overview

This project demonstrates how to create a simple zk proof using the ziskos framework. It fetches live Ethereum price data, applies basic processing logic, and generates cryptographic proofs that can be verified without revealing the underlying computation.

## The Dynamic Data Challenge

One of the key challenges in zero-knowledge proofs is handling dynamic, real-time data. Traditional ZK proofs work with static inputs, but trading bots need to process live market data that changes constantly.

### The Problem
- ZK proofs require fixed input data at proof generation time
- Market data is dynamic and changes every second
- How do we maintain data freshness while preserving proof integrity?

### The Solution: Periodic Input File Updates

This implementation solves the dynamic data challenge by treating data preparation as a recurring process rather than a one-time setup. The core idea is simple: periodically regenerate your `input.bin` file with fresh data by making `cargo build` trigger new data fetching each time.

## 📁 Project Structure

```
sha_hasher/
├── src/
│   └── main.rs          # Main ziskos program (trading logic)
├── build.rs             # Build script (data refresh mechanism)
├── Cargo.toml           # Dependencies and configuration
└── build/
    └── input.bin        # Binary input file (auto-generated)
```

## How It Works

### Approach: Periodic Input File Updates

Instead of using `build.rs` for one-time setup, we repurpose it to fetch live data every time the project builds. This works because `cargo build` automatically executes `build.rs` before compiling your main program.

#### 1. Data Fetching (`build.rs`)
```rust
// build.rs - Transforms into a data refresh mechanism for dynamic updates
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch current data from your chosen external source
    let current_data = fetch_live_data().await?;
    let timestamp = current_timestamp();
    
    // Replace existing input.bin with fresh data snapshot
    let mut file = File::create("build/input.bin")?;
    file.write_all(&current_data.to_le_bytes())?;     // Your data as bytes
    file.write_all(&timestamp.to_le_bytes())?;        // Capture timestamp
    
    Ok(())
}
```

**Purpose**: Fetches live Ethereum market data from CoinGecko API
**Trigger**: Runs automatically on every `cargo build`
**Output**: Creates `build/input.bin` with 24 bytes of market data

#### 2. Trading Logic (`src/main.rs`)
**Purpose**: Processes market data and generates trading signals
**Input**: 24 bytes from `input.bin` (price, change, timestamp)
**Output**: 4 public values (timestamp, signal, risk, price)
**Privacy**: Algorithm logic remains private

### Data Format

**Input (24 bytes):**
```
[0-7]   : ETH price in cents (u64, little-endian)
[8-15]  : 24h price change in basis points (i64, little-endian)
[16-23] : Unix timestamp (u64, little-endian)
```

**Output (4 public values):**
```
output[0] : Timestamp of analysis
output[1] : Trading signal (0=HOLD, 1=BUY, 2=SELL)
output[2] : Risk level (1=LOW, 3=HIGH)
output[3] : ETH price in dollars (rounded)
```

## Trading Algorithm

The bot implements a simple momentum-based strategy:

```rust
let signal = if price_change_24h < -500 {
    1  // BUY (price dropped >5%)
} else if price_change_24h > 300 {
    2  // SELL (price up >3%)
} else {
    0  // HOLD
};
```

**Risk Assessment:**
- **LOW (1)**: Price change < 10%
- **HIGH (3)**: Price change > 10%

## Usage

### Prerequisites
- Rust and Cargo installed
- `cargo-zisk` tool installed

### Basic Usage

```bash
# 1. Build with fresh market data
cargo build

# 2. Run the program
cargo run

# 3. Generate a zero-knowledge proof
cargo-zisk prove \
    -e target/riscv64ima-zisk-zkvm-elf/release/sha_hasher \
    -i build/input.bin \
    -o proof_$(date +%s)
```

### Automation Script for Continuous Data Updates

This creates an infinite loop with three simple steps that repeat every 60 seconds:

```bash
# Automation script for continuous data updates
while true; do
    cargo build                    # Triggers build.rs to fetch fresh data
    cargo run                      # Process the updated input.bin
    sleep 60                       # Wait before next cycle
done
```

First, `cargo build` triggers your `build.rs` to fetch fresh data and update `input.bin`. Then `cargo run` processes this updated data with your ZisK program. Finally, `sleep 60` waits for a minute before starting over. This gives you a continuously refreshed dataset every minute, with each cycle generating a new proof based on the current data snapshot.

### Proof Generation

```bash
# Continuous operation (runs every 60 seconds)
while true; do
    cargo build
    cargo-zisk prove \
        -e target/riscv64ima-zisk-zkvm-elf/release/sha_hasher \
        -i build/input.bin \
        -o proof_$(date +%s)
    sleep 60
done
```

### One-liner for Continuous Operation

```bash
# Run continuously (Ctrl+C to stop)
while true; do 
    cargo build && 
    cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/sha_hasher -i build/input.bin -o proof_$(date +%s) && 
    sleep 60
done
```

## 📡 Data Source

- **API**: CoinGecko Ethereum Price API
- **Endpoint**: `https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd&include_24hr_change=true`
- **Rate Limit**: Free tier (may hit limits with frequent builds)
- **Fallback**: Currently uses `.unwrap()` - may panic on API failures

## 🔒 Zero-Knowledge Features

### What's Public
- Trading signal (BUY/SELL/HOLD)
- Risk level assessment
- ETH price (rounded)
- Timestamp of analysis

### What's Private
- Exact price in cents
- Exact 24h change value
- Trading thresholds (-5%, +3%)
- Algorithm logic details
- Risk calculation method

## 💡 Why This Approach Works

The beauty of this method is that your main ZisK program doesn't need to change at all - it simply processes whatever current data is available in the file. This creates a seamless pipeline where each build cycle produces a fresh snapshot of real-world data, and each subsequent proof demonstrates correct computation on that specific moment in time.

The timestamp ensures verifiers can see exactly when each analysis was performed, providing full transparency about data freshness.

## 📦 Dependencies

### Build Dependencies
- `reqwest` - HTTP client for API calls
- `serde_json` - JSON parsing
- `tokio` - Async runtime

### Runtime Dependencies
- `ziskos` - Zero-knowledge proof framework
- `byteorder` - Byte order utilities (unused in current version)
- `sha2` - SHA hashing (unused in current version)

## 🛠️ Development

### Adding New Features

1. **Modify Trading Logic** (`src/main.rs`):
   ```rust
   // Add your custom algorithm here
   let signal = your_custom_logic(eth_price, price_change_24h);
   ```

2. **Add More Data Sources** (`build.rs`):
   ```rust
   // Fetch additional market data
   let volume = fetch_volume().await?;
   file.write_all(&volume.to_le_bytes())?;
   ```

3. **Extend Output** (`src/main.rs`):
   ```rust
   // Add more public outputs
   set_output(4, volume as u32);
   ```

### Error Handling

The current implementation uses `.unwrap()` which may panic on API failures. For production use, consider adding:

```rust
// In build.rs
let eth_price = data["ethereum"]["usd"]
    .as_f64()
    .ok_or("Failed to get ETH price")?;
```

## 🚨 Limitations

1. **API Rate Limits**: CoinGecko free tier has rate limits
2. **Error Handling**: Minimal error handling (may panic on API failures)
3. **Trading Strategy**: Simple momentum-based (not financial advice)
4. **Data Freshness**: Only updates on `cargo build`

## 🔮 Future Enhancements

- [ ] Add multiple data sources
- [ ] Implement more sophisticated trading algorithms
- [ ] Add proper error handling and fallbacks
- [ ] Support for multiple cryptocurrencies
- [ ] Historical data analysis
- [ ] Risk management features

## 📄 License

This project is for educational purposes. The trading algorithm is not financial advice.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with `cargo build && cargo run`
5. Submit a pull request

---

**Note**: This is a demonstration project. Do not use for actual trading without proper testing and risk management. 