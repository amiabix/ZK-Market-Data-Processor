use reqwest;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch ETH data from API
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd&include_24hr_change=true")
        .send()
        .await?;
    
    let data: Value = response.json().await?;
    
    // Extract values
    let eth_price = (data["ethereum"]["usd"].as_f64().unwrap() * 100.0) as u64;
    let price_change_24h = (data["ethereum"]["usd_24h_change"].as_f64().unwrap() * 100.0) as i64;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    
    // Write to input.bin
    fs::create_dir_all("build")?;
    let mut file = File::create("build/input.bin")?;
    
    file.write_all(&eth_price.to_le_bytes())?;
    file.write_all(&price_change_24h.to_le_bytes())?;
    file.write_all(&timestamp.to_le_bytes())?;
    
    println!("Updated: ETH ${:.2} ({:+.2}%)", 
             eth_price as f64 / 100.0, 
             price_change_24h as f64 / 100.0);
    
    Ok(())
}