use crate::{
    bridge::Bridge,
    event::{events, key::Key},
    App,
};
use anyhow::Result;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{io, time::Duration};
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

#[derive(Clone)]
pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

/// Handle tabs
impl TabsState {
    pub fn new() -> Self {
        TabsState {
            titles: vec!["Rooms".into(), "Lights".into(), "Groups".into()],

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

pub fn draw_tabs<'a>(app: &'a App) -> Tabs<'a> {
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
                .title("Press the link button on your bridge"),
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

pub async fn start_register_user_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    todo!()
}

fn exit() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

pub async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    enable_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut app_state = app.lock().unwrap();
    let events = events::EventsHandler::new(Duration::from_millis(500));
    let loader_progress = Arc::new(Mutex::new(0));

    Bridge::discover_bridges().await?;
    let loader_progress = Arc::clone(&loader_progress);

    loop {
        let tab_state = app_state.clone();
        let tabs = draw_tabs(&tab_state);
        app_state.update_user();
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
                let loader_progress = Arc::clone(&loader_progress);
                let progress_bar = draw_discovery_screen(*loader_progress.lock().unwrap());
                f.render_widget(progress_bar, chunks[0]);
            } else {
                f.render_widget(tabs, chunks[1]);
            }
        })?;
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
    // restore terminal
    exit()?;
    Ok(())
}
