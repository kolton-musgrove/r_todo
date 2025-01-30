use crate::{
    db::handler::DatabaseHandler,
    models::todo::{Priority, Todo},
    ui::edit_popup::{EditingState, InputFields},
};
use chrono::Local;
use ratatui::text::Line;
use std::time::{Duration, Instant};

#[derive(Eq, PartialEq)]
pub enum Mode {
    Normal,
    Editing,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub mode: Mode,
    pub editing_state: EditingState,
    pub selected_index: Option<usize>,
    pub show_help: bool,
    pub error_message: Option<Vec<String>>,
    pub error_shown_at: Option<Instant>,
    db: DatabaseHandler,
}

impl App {
    pub fn new(db_path: &str) -> Result<App, Box<dyn std::error::Error>> {
        let db = DatabaseHandler::new(db_path)?;
        let todos = db.load_todos()?;

        Ok(App {
            todos,
            mode: Mode::Normal,
            editing_state: EditingState {
                input_fields: InputFields {
                    text: String::new(),
                    priority: None,
                },
                selected_field: None,
            },
            selected_index: Some(0),
            show_help: false,
            error_message: None,
            error_shown_at: None,
            db,
        })
    }

    pub fn get_help_text(&self) -> Vec<Line<'_>> {
        match self.mode {
            Mode::Normal => vec![
                Line::from("Normal Mode Commands:"),
                Line::from("n - new todo"),
                Line::from("e - edit selected todo"),
                Line::from("d - delete selected todo"),
                Line::from("c - clear completed todos"),
                Line::from("h - toggle help menu"),
                Line::from("space - toggle todo completion"),
                Line::from("k/j - navigate todos"),
                Line::from("q - quit application"),
            ],
            Mode::Editing => vec![
                Line::from("Editing Mode Commands:"),
                Line::from("type to enter todo text"),
                Line::from("tab - edit next field"),
                Line::from("enter - save todo"),
                Line::from("esc - cancel editing"),
            ],
        }
    }

    pub fn set_error(&mut self, message: String) {
        let error_lines: Vec<String> = message.lines().map(String::from).collect();
        self.error_message = Some(error_lines);
        self.error_shown_at = Some(Instant::now());
    }

    pub fn check_error_timeout(&mut self) {
        if let Some(shown_at) = self.error_shown_at {
            if shown_at.elapsed() > Duration::from_secs(15) {
                self.error_message = None;
                self.error_shown_at = None;
            }
        }
    }

    pub fn add_todo(
        &mut self,
        text: String,
        priority: Priority,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo = Todo::new(text);
        todo.priority = Some(priority);

        match self.db.insert_todo(&todo) {
            Ok(id) => {
                todo.id = id;
                self.todos.push(todo);
                Ok(())
            }
            Err(e) => {
                self.set_error(format!("Failed to add todo: {}", e));
                Err(Box::new(e))
            }
        }
    }

    pub fn toggle_todo(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.completed = !todo.completed;
            todo.completed_at = if todo.completed {
                Some(Local::now())
            } else {
                None
            };

            self.db.update_todo(todo)?;
        }
        Ok(())
    }

    pub fn delete_todo(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        if index < self.todos.len() {
            let todo = &self.todos[index];
            self.db.delete_todo(todo.id)?;
            self.todos.remove(index);
            self.update_selected_index();
        }
        Ok(())
    }

    pub fn clear_completed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for todo in self.todos.iter().filter(|t| t.completed) {
            self.db.delete_todo(todo.id)?;
        }
        self.todos.retain(|todo| !todo.completed);
        self.update_selected_index();
        Ok(())
    }

    pub fn update_todo(
        &mut self,
        index: usize,
        text: String,
        priority: Priority,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.text = text;
            todo.priority = Some(priority);
            self.db.update_todo(todo)?;
        }
        Ok(())
    }

    fn update_selected_index(&mut self) {
        if let Some(selected) = self.selected_index {
            if selected >= self.todos.len() {
                self.selected_index = if self.todos.is_empty() {
                    None
                } else {
                    Some(self.todos.len() - 1)
                }
            }
        }
    }
}
