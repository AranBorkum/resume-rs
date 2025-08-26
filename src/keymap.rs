use crossterm::event::{KeyCode::Char, KeyEvent, KeyModifiers};

use crate::{employment::EmploymentEducation, state::State, ui::tabs::TabsHeadings};

pub fn global_key_map(key: KeyEvent, state: &mut State) {
    match (key.code, key.modifiers) {
        (Char('e'), KeyModifiers::NONE) => {
            state.set_selected_tab(TabsHeadings::EmploymentAndEducation)
        }
        (Char('a'), KeyModifiers::NONE) => state.set_selected_tab(TabsHeadings::AboutMe),
        (Char('c'), KeyModifiers::NONE) => state.set_selected_tab(TabsHeadings::ContactDetails),
        (Char('p'), KeyModifiers::NONE) => state.set_selected_tab(TabsHeadings::Projects),
        _ => {}
    }

    match state.selected_tab {
        TabsHeadings::EmploymentAndEducation => employment_keymap(key, state),
        TabsHeadings::AboutMe => about_me_keymap(key, state),
        _ => {}
    }
}

fn employment_keymap(key: KeyEvent, state: &mut State) {
    match state.employment_or_education {
        EmploymentEducation::Employment => match key.code {
            Char('j') => state.next_employer(),
            Char('k') => state.previous_employer(),
            _ => {}
        },
        EmploymentEducation::Education => match key.code {
            Char('j') => state.next_educator(),
            Char('k') => state.previous_educator(),
            _ => {}
        },
    }

    match (key.code, key.modifiers) {
        (Char('o'), KeyModifiers::CONTROL) => state.toggle_employment_or_education(),
        _ => {}
    }
}

fn about_me_keymap(key: KeyEvent, state: &mut State) {
    match (key.code, key.modifiers) {
        (Char('n'), KeyModifiers::CONTROL) => {
            state.scroll_about_me_down();
        }
        (Char('p'), KeyModifiers::CONTROL) => {
            state.scroll_about_me_up();
        }
        _ => {}
    }
}
