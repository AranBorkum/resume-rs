pub struct Settings {
    pub poll_duration_ms: u64,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            poll_duration_ms: 100,
        }
    }
}
