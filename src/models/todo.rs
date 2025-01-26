use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            id: 0,
            text,
            completed: false,
            created_at: Local::now(),
            completed_at: None,
        }
    }
}
