use crossterm::event::{self, KeyEvent};

use crate::{employment::EmploymentEducation, state::State, ui::tabs::TabsHeadings};

pub fn global_key_map(key: KeyEvent, state: &mut State) {
    match key.code {
        event::KeyCode::Char('e') => state.set_selected_tab(TabsHeadings::EmploymentAndEducation),
        event::KeyCode::Char('a') => state.set_selected_tab(TabsHeadings::AboutMe),
        event::KeyCode::Char('c') => state.set_selected_tab(TabsHeadings::ContactDetails),
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
            event::KeyCode::Char('j') => state.next_employer(),
            event::KeyCode::Char('k') => state.previous_employer(),
            _ => {}
        },
        EmploymentEducation::Education => match key.code {
            event::KeyCode::Char('j') => state.next_educator(),
            event::KeyCode::Char('k') => state.previous_educator(),
            _ => {}
        },
    }

    match (key.code, key.modifiers) {
        (event::KeyCode::Char('o'), event::KeyModifiers::CONTROL) => {
            state.toggle_employment_or_education()
        }
        _ => {}
    }
}

fn about_me_keymap(key: KeyEvent, state: &mut State) {
    match (key.code, key.modifiers) {
        (event::KeyCode::Char('n'), event::KeyModifiers::CONTROL) => {
            state.scroll_about_me_down();
        }
        (event::KeyCode::Char('p'), event::KeyModifiers::CONTROL) => {
            state.scroll_about_me_up();
        }
        _ => {}
    }
}
