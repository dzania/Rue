mod api;
mod bridge;
mod config;
mod errors;
use std::path::Path;
use tracing::{debug, metadata::LevelFilter};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    match Path::new(&dirs::home_dir().unwrap().join(".config/rue/rue.conf")).exists() {
        true => {
            debug!("Config file exists");
            println!("Config file exists");
        }
        false => {
            debug!("Creating config file...");
            println!("Creating config file...");
            bridge::create_user().await;
        }
    }
    Ok(())
}
