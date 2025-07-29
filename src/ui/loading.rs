use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::state::State;

pub fn render_loading_screen<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let text = format!(
        "Loading{}{}",
        ".".repeat(state.dot_count as usize),
        " ".repeat((3 - state.dot_count) as usize),
    );
    let loading = Paragraph::new(text).alignment(Alignment::Center);

    f.render_widget(loading, chunk);
}
