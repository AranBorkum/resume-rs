use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Software {
    pub backend: String,
    pub frontend: String,
    pub infrastructure: String,
}

impl Software {
    pub fn _default() -> Self {
        Self {
            backend: String::from(""),
            frontend: String::from(""),
            infrastructure: String::from(""),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Employment {
    pub employer: String,
    pub role: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
    pub software: Software,
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
            description: String::from("description"),
            software: Software::_default(),
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
