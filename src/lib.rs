use config::User;
pub mod api;
pub mod bridge;
pub mod config;
pub mod errors;
pub mod lights;
pub mod ui;

pub struct App {
    user: User,
}
