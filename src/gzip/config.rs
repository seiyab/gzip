use chrono::{DateTime, Local};

pub struct Config {
    pub mtime: DateTime<Local>,
    pub buf_size: usize,
}
