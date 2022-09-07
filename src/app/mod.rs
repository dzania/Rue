pub mod bridge;
pub mod config;
pub mod errors;
pub mod event;
pub mod handlers;
pub mod lights;
pub mod ui;

use crate::config::User;
use crate::ui::TabsState;

pub struct App {
    pub user: Option<User>,
    pub tabstate: TabsState,
}

impl App {
    pub fn new() -> Self {
        let user = match User::load() {
            Ok(user) => Some(user),
            Err(_) => None,
        };

        App {
            user,
            tabstate: TabsState::new(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
