mod api;
mod bridge;
mod config;
mod errors;
mod lights;
use std::path::Path;

#[tokio::main]
async fn main()  {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    match Path::new(&dirs::home_dir().unwrap().join(".config/rue/rue.conf")).exists() {
        true => {
            debug!("Config file exists");
            println!("Config file exists");
        }
        false => {
            bridge::create_user().await.unwrap();
        }
    }
    Ok(())
}
