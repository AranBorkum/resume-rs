use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Tabs},
    Frame,
};

#[derive(Debug, PartialEq)]
pub enum TabsHeadings {
    AboutMe,
    ContactDetails,
    EmploymentAndEducation,
}

impl TabsHeadings {
    pub fn index(&self) -> usize {
        match self {
            TabsHeadings::AboutMe => 0,
            TabsHeadings::ContactDetails => 1,
            TabsHeadings::EmploymentAndEducation => 2,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            TabsHeadings::AboutMe => "About Me (a)",
            TabsHeadings::ContactDetails => "Contact Details (c)",
            TabsHeadings::EmploymentAndEducation => "Employment and Education (e)",
        }
    }
}

pub fn render_tabs<B: Backend>(f: &mut Frame<B>, chunk: Rect, selected: usize) {
    let titles = [
        TabsHeadings::AboutMe.title(),
        TabsHeadings::ContactDetails.title(),
        TabsHeadings::EmploymentAndEducation.title(),
        "Exit (q)",
    ];
    let tab_spans: Vec<Spans> = titles.iter().map(|t| Spans::from(Span::raw(*t))).collect();

    let tabs = Tabs::new(tab_spans)
        .select(selected)
        .block(Block::default())
        .style(Style::default().fg(Color::Green))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::UNDERLINED),
        );

    f.render_widget(tabs, chunk);
}
