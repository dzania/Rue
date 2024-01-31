#![allow(unused_imports)]

use crate::{
    banner::BANNER,
    bridge::Bridge,
    event::{events, key::Key},
    App,
};
use anyhow::Result;
use color_eyre::owo_colors::{style, OwoColorize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style, Stylize},
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

/// TODO: Draw app title and version
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

    let app_state = app.lock().await;
    let events = events::EventsHandler::new(Duration::from_millis(500));
    let loader_progress = Arc::new(Mutex::new(0));

    let counter = Arc::clone(&loader_progress);
    let app_clone = Arc::clone(app);
    if app_state.user.is_none() {
        tokio::spawn(async move {
            if let Ok(_) = Bridge::create_user(bridges, counter).await {
                let mut state = app_clone.lock().await;
                state.update_user();
            };
        });
    };

    loop {
        let loader_progress = Arc::clone(&loader_progress);
        terminal.draw(|f| {
            // FIXME: move this to a function
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL))
                .gauge_style(ratatui::style::Style::default().bg(Color::Green))
                .ratio(*loader_progress.lock().unwrap() as f64 / 100 as f64);
            let banner_text = Paragraph::new(BANNER)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::new().green());
            let help_text = Paragraph::new(
                "Please press the link button on your bridge to create new user and activate app",
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(15),
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Length(5),
                ])
                .split(f.size());

            f.render_widget(banner_text, main_layout[0]);
            f.render_widget(help_text, main_layout[1]);
            f.render_widget(gauge, main_layout[2]);
        })?;
        match events.next()? {
            events::IoEvent::Input(key) => {
                if key == Key::Char('q') || key == Key::Ctrl('c') {
                    break;
                } else {
                    break;
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
