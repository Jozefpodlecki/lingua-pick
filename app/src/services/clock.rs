use chrono::{DateTime, Utc};

pub struct SystemClock;

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

impl Clock for SystemClock {

    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl SystemClock {
    pub fn new() -> Self {
        Self
    }
}