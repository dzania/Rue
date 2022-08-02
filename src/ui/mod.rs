pub mod events;
use crate::App;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Tabs, Widget},
    Terminal,
};

use std::sync::{Arc, Mutex};

pub struct Menu {Vec}

enum MenuItem {
    Lights,
    Groups,
    Rooms,
}

pub fn draw_nav() -> Result<(), io::Error> {
    todo!()
}

pub fn render_groups() -> Result<(), io::Error> {
    todo!()
}
pub fn render_lights() -> Result<(), io::Error> {
    todo!()
}

pub fn render_rooms() -> Result<(), io::Error> {
    todo!()
}
pub async fn start_ui(app: &Arc<Mutex<App>>) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);

    enable_raw_mode()?;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let menu_titles = vec!["Lights", "Groups"];
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
            f.render_widget(block, size);
            let titles = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(0);
                    Spans::from(vec![
                        Span::styled(first, Style::default().fg(Color::Yellow)),
                        Span::styled(rest, Style::default().fg(Color::Green)),
                    ])
                })
                .collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(0)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
    // restore terminal
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
