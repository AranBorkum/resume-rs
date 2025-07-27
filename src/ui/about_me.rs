use tui::{
    Frame,
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_about_me<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let about_me = Paragraph::new("About me")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(about_me, chunk);
}
