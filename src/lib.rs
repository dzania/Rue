use config::User;
use ui::TabsState;

pub mod bridge;
pub mod config;
pub mod errors;
pub mod event;
pub mod lights;
pub mod ui;

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
