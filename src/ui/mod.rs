#![allow(unused_imports)]

use crate::{
    app::Page,
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
    Frame,
};
use std::{io, time::Duration};

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
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

fn draw_tabs<'a>(app: &'a App) {
    todo!()
}

/// TODO: Draw groups page
fn draw_groups() -> Result<(), io::Error> {
    todo!()
}

/// TODO: Draw lights page
fn draw_lights() -> Result<(), io::Error> {
    todo!()
}

/// TODO: Draw rooms page
fn draw_rooms() -> Result<(), io::Error> {
    todo!()
}
/// TODO: Draw help page
fn draw_help() -> Result<(), io::Error> {
    todo!()
}

/// TODO: Draw app title and version
fn draw_title<'a>() -> Paragraph<'a> {
    todo!()
}

fn draw_discovery_screen<'a>(
    f: &mut Frame,
    app: Arc<tokio::sync::Mutex<App>>,
    bridges: Vec<Bridge>,
) -> Result<()> {
    let loader_progress = Arc::new(Mutex::new(0));
    let counter = Arc::clone(&loader_progress);
    tokio::spawn(async move {
        if let Ok(_) = Bridge::create_user(bridges, counter).await {
            let mut state = app.lock().await;
            state.update_user();
        };
    });

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
    Ok(())
}

pub fn draw_looking_for_bridge_screen<'a>(
    f: &mut Frame<'a>,
    app: &Arc<tokio::sync::Mutex<App>>,
) -> Result<()> {
    let looking_for_bridges_text =
        Span::styled("Looking for bridges...", Style::default().fg(Color::Yellow));
    let looking_for_bridges_paragraph = Paragraph::new(looking_for_bridges_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().bg(Color::Black).fg(Color::White));
    let banner_text = Paragraph::new(BANNER)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::new().green());
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
    f.render_widget(looking_for_bridges_paragraph, main_layout[1]);

    Ok(())
}

fn exit() -> Result<()> {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
async fn spawn_bridge_discovery(app: &Arc<tokio::sync::Mutex<App>>) {
    let app = Arc::clone(&app);
    tokio::spawn(async move {
        let mut app = app.lock().await;
        debug!("Searching for bridges");
        let bridges = Bridge::discover_bridges()
            .await
            .expect("Failed to find bridges");
        debug!("Setting active page");
        app.active_page = Page::Discovery(bridges);
        debug!("Active page set: {app:#?}");
    });
}

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let events = events::EventsHandler::new(Duration::from_millis(250));
    {
        let app_state = app.lock().await;
        if app_state.active_page.eq(&Page::Search) {
            spawn_bridge_discovery(app).await;
        };
    };

    loop {
        let app_state = app.lock().await;
        debug!("App State: {app_state:#?}");
        terminal.draw(|f| {
            let _ = match &app_state.active_page {
                Page::Search => draw_looking_for_bridge_screen(f, app),
                Page::Discovery(bridges) => draw_discovery_screen(f, app.clone(), bridges.to_vec()),
                _ => Ok(()),
            };
        })?;
        match events.next()? {
            events::IoEvent::Input(key) => {
                if key == Key::Char('q') || key == Key::Ctrl('c') {
                    break;
                } else {
                    break;
                }
            }
            events::IoEvent::Tick => {}
        }
    }
    //restore terminal
    exit()?;
    Ok(())
}
