use tui::{
    backend::Backend,
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{employment::EmploymentEducation, events::Event, state::State, ui::tabs::TabsHeadings};

fn style_keybind(event: &Event) -> String {
    let styled_vec = vec![
        Span::raw(" <"),
        Span::raw(event.key_binding()),
        Span::raw(": "),
        Span::raw(event.description()),
        Span::raw("> "),
    ];

    styled_vec
        .into_iter()
        .map(|s| s.content)
        .collect::<String>()
}

pub fn render_keymap<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let mut base_events = vec![
        Event::Quit,
        Event::SwitchToAboutMe,
        Event::SwitchToContactInformation,
        Event::SwitchToEmployment,
    ];

    match state.selected_tab {
        TabsHeadings::EmploymentAndEducation => match state.employment_or_education {
            EmploymentEducation::Employment => {
                base_events.push(Event::SelectEducation);
                base_events.push(Event::NextEmployer);
                base_events.push(Event::PreviousEmployer);
            }
            EmploymentEducation::Education => {
                base_events.push(Event::SelectEmployment);
                base_events.push(Event::NextEducation);
                base_events.push(Event::PreviousEducation);
            }
        },
        _ => {}
    }

    let line = base_events
        .into_iter()
        .map(|e| style_keybind(&e))
        .collect::<String>();

    let block = Paragraph::new(line).block(Block::default().borders(Borders::ALL));

    f.render_widget(block, chunk);
}
