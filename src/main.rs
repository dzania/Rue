mod bridge;
mod config;
use std::path::Path;

#[tokio::main]
async fn main() {
    match Path::new(&dirs::home_dir().unwrap().join(".config/rue/rue.conf")).exists() {
        true => {
            println!("Config file exists");
        }
        false => {
            bridge::create_user().await;
        }
    }
}
