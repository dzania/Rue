use config::User;
use ui::TabsState;

pub mod api;
pub mod bridge;
pub mod config;
pub mod errors;
pub mod event;
pub mod lights;
pub mod ui;

pub struct App {
    user: Option<User>,
    tabstate: TabsState,
}

impl App {
    pub async fn new() -> Self {
        let user = match User::load().await {
            Ok(user) => Some(user),
            Err(e) => None,
        };

        App {
            user,
            tabstate: TabsState::new(),
        }
    }
}
