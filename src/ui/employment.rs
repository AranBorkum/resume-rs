use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::{employment::EmploymentEducation, state::State};

fn employment_block<'a>(list_state: &mut ListState, state: &State) -> List<'a> {
    list_state.select(Some(state.selected_employment_entry));
    let employment_list_items: Vec<ListItem> = state
        .employment_history
        .iter()
        .enumerate()
        .map(|(i, s)| {
            ListItem::new(
                s.representation(
                    i == state.selected_employment_entry
                        && state.employment_or_education == EmploymentEducation::Employment,
                )
                .clone(),
            )
        })
        .collect();

    let mut block = Block::default().title("Employment").borders(Borders::ALL);
    if state.employment_or_education == EmploymentEducation::Employment {
        block = block.border_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
    }

    let mut list = List::new(employment_list_items).block(block);

    if state.employment_or_education == EmploymentEducation::Employment {
        list = list.highlight_style(Style::default().add_modifier(Modifier::BOLD));
    }

    list
}

fn education_block<'a>(list_state: &mut ListState, state: &State) -> List<'a> {
    list_state.select(Some(state.selected_education_entry));
    let education_list_items: Vec<ListItem> = state
        .education_history
        .iter()
        .enumerate()
        .map(|(i, s)| {
            ListItem::new(
                s.representation(
                    i == state.selected_education_entry
                        && state.employment_or_education == EmploymentEducation::Education,
                )
                .clone(),
            )
        })
        .collect();

    let mut block = Block::default().title("Education").borders(Borders::ALL);
    if state.employment_or_education == EmploymentEducation::Education {
        block = block.border_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
    }

    let mut list = List::new(education_list_items).block(block);

    if state.employment_or_education == EmploymentEducation::Education {
        list = list.highlight_style(Style::default().add_modifier(Modifier::BOLD));
    }

    list
}

fn description_block<'a, B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    match state.employment_or_education {
        EmploymentEducation::Employment => employment_description_block(f, chunk, state),
        EmploymentEducation::Education => education_description_block(f, chunk, state),
    }
}

fn employment_description_block<'a, B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let description_section = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(6),
        ])
        .split(chunk);

    let employment_entry = state.get_employment_entry();

    let title_text = format!(
        "{}\n{}\n{} - {}",
        employment_entry.employer,
        employment_entry.role,
        employment_entry.start_date,
        employment_entry.end_date
    );
    let software_text = format!(
        "\nSoftware Stack\nBackend: {}\nFrontend: {}\nInfrastructre: {}",
        employment_entry.software.backend,
        employment_entry.software.frontend,
        employment_entry.software.infrastructure
    );

    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::NONE));

    let details = Paragraph::new(employment_entry.description.clone())
        .style(Style::default().add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::NONE));

    let software = Paragraph::new(software_text)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(title, description_section[0]);
    f.render_widget(details, description_section[1]);
    f.render_widget(software, description_section[2]);
}

fn education_description_block<'a, B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let description_section = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(6),
        ])
        .split(chunk);

    let education_entry = state.get_education_entry();

    let title_text = format!(
        "{}\n{}\n{} - {}",
        education_entry.educator,
        education_entry.qualification,
        education_entry.start_date,
        education_entry.end_date
    );

    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::NONE));

    let details = Paragraph::new(education_entry.description.clone())
        .style(Style::default().add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(title, description_section[0]);
    f.render_widget(details, description_section[1]);
}

pub fn render_employment<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let top_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunk);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(top_bottom[1]);

    let list_chuncks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(bottom_chunks[0]);

    let title = Paragraph::new("Employment and Education History")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    let mut employment_list_state = ListState::default();
    let employment_list = employment_block(&mut employment_list_state, &state);

    let mut education_list_state = ListState::default();
    let education_list = education_block(&mut education_list_state, &state);

    f.render_widget(title, top_bottom[0]);
    f.render_stateful_widget(employment_list, list_chuncks[0], &mut employment_list_state);
    f.render_stateful_widget(education_list, list_chuncks[1], &mut education_list_state);
    description_block(f, bottom_chunks[2], state);
}
