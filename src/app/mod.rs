use crate::config::User;
use crate::ui::TabsState;

pub struct App {
    pub user: Option<User>,
    pub tabstate: TabsState,
    pub bridge_discover_progress: u32,
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
            bridge_discover_progress: 0,
        }
    }
    pub fn authorized(&self) -> bool {
        self.user.is_some()
    }

    pub fn update_on_tick(&self) {}
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
