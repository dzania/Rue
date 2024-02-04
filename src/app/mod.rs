use crate::bridge::Bridge;
use crate::config::User;
use crate::ui::TabsState;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Page {
    Discovery(Vec<Bridge>),
    Rooms,
    Lights,
    Search,
}

#[derive(Clone, Debug)]
pub struct App {
    pub user: Option<User>,
    pub tabstate: TabsState,
    pub active_page: Page,
}

impl App {
    pub fn new() -> Self {
        let user = match User::load() {
            Ok(user) => Some(user),
            Err(_) => None,
        };

        let active_page = if user.is_some() {
            Page::Rooms
        } else {
            Page::Search
        };

        App {
            user,
            tabstate: TabsState::new(),
            active_page,
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
