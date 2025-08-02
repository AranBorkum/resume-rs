pub struct Settings {
    pub poll_duration_ms: u64,
    pub aws_bucket: String,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            poll_duration_ms: 100,
            aws_bucket: String::from("rusty-resume-s3-bucket"),
        }
    }
}
