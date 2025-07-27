use tui::{
    Frame,
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::assets::banner;

pub fn render_banner<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let block = Paragraph::new(banner::ASCII_ART)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(block, chunk);
}
