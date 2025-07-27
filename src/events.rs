#[derive(Clone)]
pub enum Event {
    Quit,
    SwitchToAboutMe,
    SwitchToContactInformation,
    SwitchToEmployment,

    NextEmployer,
    PreviousEmployer,
    NextEducation,
    PreviousEducation,
    SelectEducation,
    SelectEmployment,
}

impl Event {
    pub fn description(&self) -> &str {
        match self {
            Self::Quit => "quit",
            Self::SwitchToAboutMe => "about me",
            Self::SwitchToContactInformation => "contact",
            Self::SwitchToEmployment => "employment",
            Self::NextEmployer => "next employer",
            Self::PreviousEmployer => "previous employer",
            Self::NextEducation => "next education",
            Self::PreviousEducation => "previous education",
            Self::SelectEducation => "select education",
            Self::SelectEmployment => "select employment",
        }
    }

    pub fn key_binding(&self) -> &str {
        match self {
            Self::Quit => "q",
            Self::SwitchToAboutMe => "a",
            Self::SwitchToContactInformation => "c",
            Self::SwitchToEmployment => "e",
            Self::NextEmployer => "C-n",
            Self::PreviousEmployer => "C-p",
            Self::NextEducation => "C-n",
            Self::PreviousEducation => "C-p",
            Self::SelectEducation => "C-o",
            Self::SelectEmployment => "C-o",
        }
    }
}
