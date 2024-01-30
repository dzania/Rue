use crate::{
    banner::BANNER,
    bridge::Bridge,
    event::{events, key::Key},
    App,
};
use anyhow::Result;
use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    style::Colors,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    text::Span,
    widgets::{Block, Borders, Gauge, Paragraph},
};
use std::{io, time::Duration};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TabsState {
    pub pages: Vec<String>,
    pub index: usize,
}

/// Handle tabs
impl TabsState {
    pub fn new() -> Self {
        TabsState {
            pages: vec!["Rooms".into(), "Lights".into(), "Groups".into()],

            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.pages.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.pages.len() - 1;
        }
    }
}

impl Default for TabsState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn draw_tabs<'a>(app: &'a App) {
    todo!()
}

/// TODO: Draw groups page
pub fn draw_groups() -> Result<(), io::Error> {
    todo!()
}

/// TODO: Draw lights page
pub fn draw_lights() -> Result<(), io::Error> {
    todo!()
}

/// TODO: Draw rooms page
pub fn draw_rooms() -> Result<(), io::Error> {
    todo!()
}
/// TODO: Draw help page
pub fn draw_help() -> Result<(), io::Error> {
    todo!()
}

/// Draw app title and version
fn draw_title<'a>() -> Paragraph<'a> {
    todo!()
}

pub fn draw_discovery_screen<'a>(counter: u64) {
    todo!()
}

pub async fn start_register_user_ui(_app: &Arc<Mutex<App>>) -> Result<()> {
    todo!()
}

fn exit() -> Result<()> {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>, bridges: Vec<Bridge>) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app_state = app.lock().await;
    let events = events::EventsHandler::new(Duration::from_millis(500));
    let loader_progress = Arc::new(Mutex::new(0));

    let counter = Arc::clone(&loader_progress);
    if app_state.user.is_none() {
        tokio::spawn(async move { Bridge::create_user(bridges, counter).await });
    }

    loop {
        let loader_progress = Arc::clone(&loader_progress);
        terminal.draw(|f| {
            let gauge = Gauge::default()
                .block(Block::default().title("Progress").borders(Borders::ALL))
                .gauge_style(ratatui::style::Style::default())
                .ratio(*loader_progress.lock().unwrap() as f64 / 10 as f64);
            let row4 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(8),
                        Constraint::Length(0 as u16 + 2),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            f.render_widget(gauge, row4[0]);
        })?;
        app_state.update_user();
        match events.next()? {
            events::IoEvent::Input(key) => {
                if key == Key::Char('q') {
                    break;
                } else {
                    todo!();
                }
            }
            events::IoEvent::Tick => {
                app_state.update_on_tick();
            }
        }
    }
    //restore terminal
    exit()?;
    Ok(())
}
