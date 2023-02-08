use crate::App;

use crate::event::{events, key::Key};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Borders, LineGauge, Paragraph, Tabs},
    Terminal,
};

use std::sync::{Arc, Mutex};

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

/// Handle tabs
impl TabsState {
    pub fn new() -> Self {
        TabsState {
            titles: vec![
                "Rooms".into(),
                "Lights".into(),
                "Groups".into(),
                "Help".into(),
            ],

            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

impl Default for TabsState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn draw_tabs(app: &App) -> Tabs {
    let tabs = app
        .tabstate
        .titles
        .iter()
        .map(|t| {
            Spans::from(vec![Span::styled(
                t,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            )])
        })
        .collect();
    Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .select(app.tabstate.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        )
}

/// Draw groups page
/// TODO: get all groups
pub fn draw_groups() -> Result<(), io::Error> {
    todo!()
}

/// Draw lights page
/// TODO: get all lights
pub fn draw_lights() -> Result<(), io::Error> {
    todo!()
}

/// Draw rooms page
/// Create client
pub fn draw_rooms() -> Result<(), io::Error> {
    todo!()
}
/// Draw help page
/// Create client
pub fn draw_help() -> Result<(), io::Error> {
    todo!()
}

/// Draw app title and version
fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rue")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE))
}

pub fn draw_discovery_screen<'a>(counter: u64) -> LineGauge<'a> {
    let sec = Duration::from_secs(counter).as_secs();
    let ratio = sec as f64 / 100.0;
    LineGauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Looking for bridges"),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .line_set(symbols::line::THICK)
        .ratio(ratio)
}

pub async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    enable_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut app_state = app.lock().unwrap();
    let events = events::EventsHandler::new(Duration::from_millis(2));

    loop {
        let tabs = draw_tabs(&app_state);
        // do we need multiple terminal draw?
        terminal.draw(|f| {
            let title = draw_title();
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);
            f.render_widget(title, chunks[0]);

            if app_state.user.is_none() {
                let progress = draw_discovery_screen(1);
                f.render_widget(progress, chunks[0]);
            } else {
                f.render_widget(tabs, chunks[1]);
            }
        })?;
        match events.next()? {
            events::IoEvent::Input(key) => {
                if key == Key::Char('q') {
                    break;
                }
            }

            events::IoEvent::Tick => {
                app_state.update_on_tick();
            }
        }
    }
    // restore terminal
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
