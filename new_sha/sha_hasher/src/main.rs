#![no_main]
ziskos::entrypoint!(main);

use std::convert::TryInto;
use ziskos::{read_input, set_output};

fn main() {
    let input: Vec<u8> = read_input();
    
    // Parse the data from input.bin
    let eth_price = u64::from_le_bytes(input[0..8].try_into().unwrap());
    let price_change_24h = i64::from_le_bytes(input[8..16].try_into().unwrap());
    let timestamp = u64::from_le_bytes(input[16..24].try_into().unwrap());
    
    // Simple trading logic
    let signal = if price_change_24h < -500 {
        1  // BUY (price dropped >5%)
    } else if price_change_24h > 300 {
        2  // SELL (price up >3%)
    } else {
        0  // HOLD
    };
    
    // Risk level based on volatility
    let risk = if price_change_24h.abs() > 1000 { 3 } else { 1 };
    
    // Make results public
    set_output(0, timestamp as u32);
    set_output(1, signal as u32);
    set_output(2, risk as u32);
    set_output(3, (eth_price / 100) as u32);  // ETH price in dollars
}