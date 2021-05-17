mod config;
mod dns;

use config::AppConfig;
use std::error::Error;

fn query(config: AppConfig) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() {
    let config = AppConfig::from(&mut std::env::args_os());
    
    if let Err(e) = query(config) {
        eprintln!("Error performing DNS query: {}", e);
    }
}
