use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

    let mut list = List::new(employment_list_items)
        .block(block);

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

    let mut list = List::new(education_list_items)
        .block(block);

    if state.employment_or_education == EmploymentEducation::Education {
        list = list.highlight_style(Style::default().add_modifier(Modifier::BOLD));
    }

    list
}

pub fn render_employment<B: Backend>(f: &mut Frame<B>, chunk: Rect, state: &State) {
    let top_bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
        .split(chunk);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
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

    let description = Block::default();

    f.render_widget(title, top_bottom[0]);
    f.render_stateful_widget(employment_list, list_chuncks[0], &mut employment_list_state);
    f.render_stateful_widget(education_list, list_chuncks[1], &mut education_list_state);
    f.render_widget(description, bottom_chunks[1]);
}
