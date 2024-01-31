use anyhow::Result;
use app::App;
use banner::BANNER;
use bridge::Bridge;
use std::io;
use std::sync::Arc;

pub mod app;
pub mod banner;
pub mod bridge;
pub mod config;
pub mod errors;
pub mod event;
pub mod handlers;
pub mod lights;
pub mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Arc::new(tokio::sync::Mutex::new(App::new()));
    println!("{}", BANNER);
    println!("Looking for bridges..");
    let bridges = Bridge::discover_bridges().await?;
    //println!(
        //"Found {} bridges do you want to authorize? y/n",
        //&bridges.len()
    //);
    //let mut user_input = String::new();
    //let stdin = io::stdin();
    //stdin.read_line(&mut user_input)?;
    ui::start_ui(&app, bridges).await?;
    Ok(())
}
