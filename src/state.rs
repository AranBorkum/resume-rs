use crate::{
    employment::{Education, Employment, EmploymentEducation, KRAKEN, MPB, SOCIUS, UOS_MPHYS, UOS_PHD},
    ui::tabs::TabsHeadings,
};

pub struct State<'a> {
    pub selected_tab: TabsHeadings,
    pub selected_employment_entry: usize,
    pub employment_history: Vec<Employment<'a>>,
    pub selected_education_entry: usize,
    pub education_history: Vec<Education<'a>>,
    pub employment_or_education: EmploymentEducation,
}

impl State<'_> {
    pub fn default() -> Self {
        Self {
            selected_tab: TabsHeadings::AboutMe,
            selected_employment_entry: 0,
            employment_history: vec![KRAKEN, MPB, SOCIUS],
            selected_education_entry: 0,
            education_history: vec![UOS_PHD, UOS_MPHYS],
            employment_or_education: EmploymentEducation::Employment,
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
        let mut state = State::default();
        assert_eq!(state.selected_employment_entry, 0);
        state.next_employer();
        assert_eq!(state.selected_employment_entry, 1);
    }

    #[test]
    fn test_previous_employer() {
        let mut state = State::default();
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
            employment_history: vec![KRAKEN, MPB],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![UOS_PHD, UOS_MPHYS],
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
            employment_history: vec![KRAKEN, MPB],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![UOS_PHD, UOS_MPHYS],
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
        let mut state = State::default();
        assert_eq!(state.selected_education_entry, 0);
        state.next_educator();
        assert_eq!(state.selected_education_entry, 1);
    }

    #[test]
    fn test_previous_educator() {
        let mut state = State::default();
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
            employment_history: vec![KRAKEN, MPB],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![UOS_PHD, UOS_MPHYS],
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
            employment_history: vec![KRAKEN, MPB],
            employment_or_education: EmploymentEducation::Employment,
            selected_education_entry: 1,
            education_history: vec![UOS_PHD, UOS_MPHYS],
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
}
