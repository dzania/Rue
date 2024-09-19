use std::sync::Arc;
use tokio::task;

use crate::bridge::Bridge;
use crate::config::User;
use crate::errors::ConfigError;
use crate::ui::TabsState;

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
    pub discovery_loader: u32,
    pub discovery_task_spawned: bool,
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
            discovery_loader: 0,
            discovery_task_spawned: false,
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

    pub fn increment_loader(&mut self) {
        self.discovery_loader += 4;
    }

    // TODO: Handle reading current lights status here
    pub fn update_on_tick(&self) {}
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub fn spawn_discovery_task(
    app: &Arc<tokio::sync::Mutex<App>>,
    bridges: Vec<Bridge>,
) -> Result<(), ConfigError> {
    let app_clone = Arc::clone(&app);
    task::spawn(async move {
        for _ in 0..25 {
            let mut state = app_clone.lock().await;
            match Bridge::create_user(bridges.clone()).await {
                Ok(user) => {
                    user.save().await.expect("Failed to save user");
                    break;
                }
                Err(e) => {
                    error!("Error creating user: {:?}", e);
                }
            }
            debug!("Incrementing loader");
            state.increment_loader();
            state.discovery_task_spawned = true;
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });
    Ok(())
}
