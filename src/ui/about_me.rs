use std::env;

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use serde::Deserialize;

use crate::state::State;

#[derive(Debug, Deserialize)]
struct AboutMe {
    pub about_me: String,
}

impl AboutMe {
    pub fn default() -> Self {
        Self {
            about_me: String::from(""),
        }
    }

    pub fn from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let cwd = env::current_dir()?;
        let file_path = cwd.join("data/about_me.json");
        let json_data = std::fs::read_to_string(file_path)?;
        let about_me: Self = serde_json::from_str(&json_data)?;
        Ok(about_me)
    }
}

pub fn render_about_me<B: Backend>(f: &mut Frame, chunk: Rect, state: &State) {
    let top_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunk);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Max(80), Constraint::Min(0)])
        .split(top_bottom[1])[1];

    let me = match AboutMe::from_file() {
        Ok(me) => me,
        Err(_) => AboutMe::default(),
    };

    let title = Paragraph::new("About me")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    let about_me = Paragraph::new(me.about_me)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((state.about_me_scroll_offset, 0))
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(title, top_bottom[0]);
    f.render_widget(about_me, middle);
}
