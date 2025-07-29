use std::{
    env,
    time::{Duration, Instant},
};

use crate::{
    employment::{Education, Employment, EmploymentEducation},
    ui::tabs::TabsHeadings,
};

pub struct State {
    pub selected_tab: TabsHeadings,
    pub selected_employment_entry: usize,
    pub employment_history: Vec<Employment>,
    pub selected_education_entry: usize,
    pub education_history: Vec<Education>,
    pub employment_or_education: EmploymentEducation,
    pub about_me_scroll_offset: u16,
    last_tick: Instant,
    pub dot_count: u8,
    pub is_loading: bool,
}

impl State {
    pub fn default() -> Self {
        Self {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 0,
            employment_history: Vec::new(),
            selected_education_entry: 0,
            education_history: Vec::new(),
            employment_or_education: EmploymentEducation::Employment,
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        }
    }

    pub fn set_selected_tab(&mut self, tab: TabsHeadings) {
        self.selected_tab = tab;
    }

    pub fn next_employer(&mut self) {
        if self.selected_employment_entry != self.employment_history.len() - 1 {
            self.selected_employment_entry = self.selected_employment_entry + 1
        }
    }

    pub fn previous_employer(&mut self) {
        if self.selected_employment_entry != 0 {
            self.selected_employment_entry = self.selected_employment_entry - 1
        }
    }

    pub fn next_educator(&mut self) {
        if self.selected_education_entry != self.education_history.len() - 1 {
            self.selected_education_entry = self.selected_education_entry + 1
        }
    }

    pub fn previous_educator(&mut self) {
        if self.selected_education_entry != 0 {
            self.selected_education_entry = self.selected_education_entry - 1
        }
    }

    pub fn toggle_employment_or_education(&mut self) {
        self.employment_or_education = match self.employment_or_education {
            EmploymentEducation::Employment => EmploymentEducation::Education,
            EmploymentEducation::Education => EmploymentEducation::Employment,
        }
    }

    pub fn load_employment_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = env::current_dir()?;
        let file_path = cwd.join("data/employment.json");
        let json_data = std::fs::read_to_string(file_path)?;
        let employment: Vec<Employment> = serde_json::from_str(&json_data)?;
        self.employment_history = employment;
        Ok(())
    }

    pub fn load_education_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cwd = env::current_dir()?;
        let file_path = cwd.join("data/education.json");
        let json_data = std::fs::read_to_string(file_path)?;
        let education: Vec<Education> = serde_json::from_str(&json_data)?;
        self.education_history = education;
        Ok(())
    }

    pub fn scroll_about_me_down(&mut self) {
        self.about_me_scroll_offset = self.about_me_scroll_offset + 1;
    }

    pub fn scroll_about_me_up(&mut self) {
        if self.about_me_scroll_offset > 0 {
            self.about_me_scroll_offset = self.about_me_scroll_offset - 1;
        }
    }

    pub fn update_dot_count(&mut self) {
        if self.last_tick.elapsed() >= Duration::from_millis(500) {
            self.dot_count = (self.dot_count + 1) % 4; // 0, 1, 2, 3 (will show up to 3 dots)
            self.last_tick = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_selected_tab() {
        let mut state = State::default();
        assert_eq!(state.selected_tab, TabsHeadings::AboutMe);
        state.set_selected_tab(TabsHeadings::EmploymentAndEducation);
        assert_eq!(state.selected_tab, TabsHeadings::EmploymentAndEducation);
    }

    #[test]
    fn test_previous_employer_from_zero() {
        let mut state = State::default();
        assert_eq!(state.selected_employment_entry, 0);
        state.previous_employer();
        assert_eq!(state.selected_employment_entry, 0);
    }

    #[test]
    fn test_next_employer() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 0,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_employment_entry, 0);
        state.next_employer();
        assert_eq!(state.selected_employment_entry, 1);
    }

    #[test]
    fn test_previous_employer() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 0,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_employment_entry, 0);
        state.next_employer();
        assert_eq!(state.selected_employment_entry, 1);
        state.previous_employer();
        assert_eq!(state.selected_employment_entry, 0);
    }

    #[test]
    fn test_next_employer_from_max() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_employment_entry, 1);
        state.next_employer();
        assert_eq!(state.selected_employment_entry, 1);
    }

    #[test]
    fn test_previous_employer_from_max() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_employment_entry, 1);
        state.next_employer();
        assert_eq!(state.selected_employment_entry, 1);
        state.previous_employer();
        assert_eq!(state.selected_employment_entry, 0);
    }

    #[test]
    fn test_previous_education_from_zero() {
        let mut state = State::default();
        assert_eq!(state.selected_education_entry, 0);
        state.previous_educator();
        assert_eq!(state.selected_education_entry, 0);
    }

    #[test]
    fn test_next_educator() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 0,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_education_entry, 0);
        state.next_educator();
        assert_eq!(state.selected_education_entry, 1);
    }

    #[test]
    fn test_previous_educator() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 0,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_education_entry, 0);
        state.next_educator();
        assert_eq!(state.selected_education_entry, 1);
        state.previous_educator();
        assert_eq!(state.selected_education_entry, 0);
    }

    #[test]
    fn test_next_educator_from_max() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_education_entry, 1);
        state.next_educator();
        assert_eq!(state.selected_education_entry, 1);
    }

    #[test]
    fn test_previous_educator_from_max() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 0,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.selected_education_entry, 1);
        state.next_educator();
        assert_eq!(state.selected_education_entry, 1);
        state.previous_educator();
        assert_eq!(state.selected_education_entry, 0);
    }

    #[test]
    fn test_toggle_employment_or_education() {
        let mut state = State::default();
        assert_eq!(
            state.employment_or_education,
            EmploymentEducation::Employment
        );
        state.toggle_employment_or_education();
        assert_eq!(
            state.employment_or_education,
            EmploymentEducation::Education
        );
        state.toggle_employment_or_education();
        assert_eq!(
            state.employment_or_education,
            EmploymentEducation::Employment
        );
    }

    #[test]
    fn test_scroll_about_me_down() {
        let mut state = State::default();
        assert_eq!(state.about_me_scroll_offset, 0);
        state.scroll_about_me_down();
        assert_eq!(state.about_me_scroll_offset, 1);
    }

    #[test]
    fn test_scroll_about_me_up() {
        let mut state = State {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 1,
            employment_history: vec![Employment::_default(), Employment::_default()],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![Education::_default(), Education::_default()],
            about_me_scroll_offset: 1,
            last_tick: Instant::now(),
            dot_count: 1,
            is_loading: false,
        };
        assert_eq!(state.about_me_scroll_offset, 1);
        state.scroll_about_me_up();
        assert_eq!(state.about_me_scroll_offset, 0);
    }

    #[test]
    fn test_scroll_about_me_up_does_not_underflow() {
        let mut state = State::default();
        assert_eq!(state.about_me_scroll_offset, 0);
        state.scroll_about_me_up();
        assert_eq!(state.about_me_scroll_offset, 0);
    }
}
