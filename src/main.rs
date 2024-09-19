use anyhow::Result;
use app::App;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
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

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    // Clean the log file
    let log_file_path = "app.log";
    // Set up logging to file
    let file_appender = FileAppender::builder()
        .append(false)
        .build(log_file_path)
        .expect("Failed to create file appender.");

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("file").build(LevelFilter::Trace))
        .expect("Failed to build log configuration.");

    log4rs::init_config(config).expect("Failed to initialize logging configuration.");

    // Start the application
    debug!("Creating app");
    let app = Arc::new(tokio::sync::Mutex::new(App::new()));
    debug!("Starting ui");
    ui::start_ui(&app).await?;
    Ok(())
}
