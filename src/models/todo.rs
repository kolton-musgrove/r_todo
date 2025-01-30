use chrono::{DateTime, Local};
use std::fmt::Display;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
    pub priority: Option<Priority>,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            id: 0,
            text,
            completed: false,
            created_at: Local::now(),
            completed_at: None,
            priority: None,
        }
    }
}
