use crate::config::User;
use crate::ui::TabsState;

#[derive(Clone)]
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
    // Reload user
    pub fn update_user(&mut self) {
        let user = match User::load() {
            Ok(user) => Some(user),
            Err(_) => None,
        };
        self.user = user;
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
