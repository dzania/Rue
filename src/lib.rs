use config::User;

pub mod api;
pub mod bridge;
pub mod config;
pub mod errors;
pub mod event;
pub mod lights;
pub mod ui;

pub struct App {
    user: Option<User>,
}

impl App {}
