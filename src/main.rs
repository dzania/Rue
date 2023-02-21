use app::App;
use std::sync::{Arc, Mutex};

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
async fn main() {
    println!("{}", banner::BANNER);
    //let app = Arc::new(Mutex::new(App::new()));
    //ui::start_ui(&app).await.unwrap();
}
