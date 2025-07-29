use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Employment {
    pub employer: String,
    pub role: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct Education {
    pub educator: String,
    pub qualification: String,
    pub start_date: String,
    pub end_date: String,
}

impl Employment {
    pub fn representation(&self, selected: bool) -> String {
        let prefix = match selected {
            true => " * ",
            false => "   ",
        };
        format!(
            "   {}\n{}{}\n   {} - {}\n ",
            self.employer, prefix, self.role, self.start_date, self.end_date
        )
    }

    pub fn _default() -> Self {
        Self {
            employer: String::from("default"),
            role: String::from("role"),
            start_date: String::from("start date"),
            end_date: String::from("end date"),
        }
    }
}

impl Education {
    pub fn representation(&self, selected: bool) -> String {
        let prefix = match selected {
            true => " * ",
            false => "   ",
        };
        format!(
            "   {}\n{}{}\n   {} - {}\n ",
            self.educator, prefix, self.qualification, self.start_date, self.end_date
        )
    }

    pub fn _default() -> Self {
        Self {
            educator: String::from("default"),
            qualification: String::from("qualification"),
            start_date: String::from("start date"),
            end_date: String::from("end date"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EmploymentEducation {
    Employment,
    Education,
}
