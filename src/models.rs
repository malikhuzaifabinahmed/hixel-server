use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeTracker {
    pub shift_duration: Duration,
    pub active_duration: Duration,
    pub idle_duration: Duration,
    pub break_duration: Duration,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShiftData {
    pub id: u32,
    pub date: DateTime<Utc>,
    pub name: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub total_duration: Duration,
    pub time_tracker: TimeTracker,
    pub last_break_start: Option<DateTime<Utc>>,
}
