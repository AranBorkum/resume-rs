pub struct Employment<'a> {
    pub employer: &'a str,
    pub role: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
}

pub struct Education<'a> {
    pub educator: &'a str,
    pub qualification: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
}

impl Employment<'_> {
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
}

impl Education<'_> {
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
}

#[derive(Debug, PartialEq)]
pub enum EmploymentEducation {
    Employment,
    Education,
}

pub const KRAKEN: Employment = Employment {
    employer: "Kraken",
    role: "Software Engineer II",
    start_date: "Dec 2024",
    end_date: "Present",
};
pub const MPB: Employment = Employment {
    employer: "MPB.com",
    role: "Software Engineer",
    start_date: "Apr 2023",
    end_date: "Dec 2024",
};
pub const SOCIUS: Employment = Employment {
    employer: "Socius Ventures",
    role: "Software Engineer",
    start_date: "Oct 2022",
    end_date: "Apr 2023",
};

pub const UOS_PHD: Education = Education {
    educator: "University of Sussex",
    qualification: "Ph.D Physics",
    start_date: "Sep 2018",
    end_date: "Dec 2022",
};
pub const UOS_MPHYS: Education = Education {
    educator: "University of Sussex",
    qualification: "MPhys Physics",
    start_date: "Sep 2014",
    end_date: "July 2018",
};
