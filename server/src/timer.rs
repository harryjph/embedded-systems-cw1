use chrono::{DateTime, Utc};

pub trait Timer {
    fn get_time(&self) -> DateTime<Utc>;
}

pub struct RealTimer {}

impl RealTimer {
    pub fn new() -> Self {
        RealTimer {}
    }
}

impl Timer for RealTimer {
    fn get_time(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
