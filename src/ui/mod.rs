use crate::App;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
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

pub fn draw_tabs(app: &App) -> Result<Tabs, io::Error> {
    let tabs = app
        .tabstate
        .titles
        .into_iter()
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
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tabstate.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        ))
}

pub fn draw_groups() -> Result<(), io::Error> {
    todo!()
}
pub fn draw_lights() -> Result<(), io::Error> {
    todo!()
}

pub fn draw_rooms() -> Result<(), io::Error> {
    todo!()
}
pub fn draw_help() -> Result<(), io::Error> {
    todo!()
}

// Draw app title
fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rue")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
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
                //KeyCode::Right => app.tabstate.next(),
                //KeyCode::Char('l') => app.tabstate.next(),
                //KeyCode::Left => app.tabstate.previous(),
                //KeyCode::Char('h') => app.tabstate.previous(),
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
