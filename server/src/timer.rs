use chrono::{DateTime, TimeZone, Utc};

pub trait Timer {
    fn get_time() -> DateTime<Utc>;
}

pub struct RealTimer {}

impl RealTimer {
    pub fn new() -> Self {
        RealTimer {}
    }
}

impl Timer for RealTimer {
    fn get_time() -> DateTime<Utc> {
        Utc::now()
    }
}
