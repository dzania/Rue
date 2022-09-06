use crate::event::{
    events::{Events, IoEvent},
    key::Key,
};
use crate::App;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};

use std::sync::{Arc, Mutex};

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

// Handle tabs
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

pub fn draw_tabs(app: &App) -> Result<Tabs, io::Error> {
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
    Ok(Tabs::new(tabs)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .select(app.tabstate.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        ))
}

/// Draw groups page
pub fn draw_groups() -> Result<(), io::Error> {
    todo!()
}

/// Draw lights page
pub fn draw_lights() -> Result<(), io::Error> {
    todo!()
}

/// Draw rooms page
pub fn draw_rooms() -> Result<(), io::Error> {
    todo!()
}
/// Draw help page
pub fn draw_help() -> Result<(), io::Error> {
    todo!()
}

/// Draw app title
fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rue")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE))
}

pub async fn start_ui(app: &Arc<Mutex<App>>) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    enable_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut app_state = app.lock().unwrap();
    let events = Events::new(Duration::from_millis(250));
    loop {
        let tabs = draw_tabs(&app_state)?;
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let title = draw_title();
            f.render_widget(title, chunks[0]);
            f.render_widget(tabs, chunks[1]);
        })?;
        match events.next().unwrap() {
            IoEvent::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                } else {
                    handlers::handle_key_events(key, &mut app).await;
                }
            }
            IoEvent::Tick => {
                app.tick_update();
            }
        }
    }
    // restore terminal
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
