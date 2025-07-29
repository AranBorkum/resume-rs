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

    ScrollAboutMeTextDown,
    ScrollAboutMeTextUp,

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
            Self::ScrollAboutMeTextDown => "scroll down",
            Self::ScrollAboutMeTextUp => "scroll up",
        }
    }

    pub fn key_binding(&self) -> &str {
        match self {
            Self::Quit => "q",
            Self::SwitchToAboutMe => "a",
            Self::SwitchToContactInformation => "c",
            Self::SwitchToEmployment => "e",
            Self::NextEmployer => "j",
            Self::PreviousEmployer => "k",
            Self::NextEducation => "j",
            Self::PreviousEducation => "k",
            Self::SelectEducation => "C-o",
            Self::SelectEmployment => "C-o",
            Self::ScrollAboutMeTextDown => "C-n",
            Self::ScrollAboutMeTextUp => "C-p",
        }
    }
}
