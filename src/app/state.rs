use crate::{
    db::handler::DatabaseHandler,
    models::todo::{Priority, Todo},
    ui::edit_popup::{EditingState, InputFields},
};
use chrono::Local;
use ratatui::{text::Line, widgets::TableState};
use std::time::{Duration, Instant};

#[derive(Eq, PartialEq)]
pub enum Mode {
    Normal,
    Editing,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortCriteria {
    Priority,
    Completed,
    CreatedDate,
}

pub struct App {
    pub todos: Vec<Todo>,
    pub mode: Mode,
    pub table_state: TableState,
    pub editing_state: EditingState,
    pub sort_by: SortCriteria,
    pub sort_asc: bool,
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
            table_state: TableState::default().with_selected(0),
            editing_state: EditingState {
                input_fields: InputFields {
                    text: String::new(),
                    priority: None,
                },
                selected_field: None,
            },
            sort_by: SortCriteria::Priority,
            sort_asc: true,
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
        }
        Ok(())
    }

    pub fn clear_completed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for todo in self.todos.iter().filter(|t| t.completed) {
            self.db.delete_todo(todo.id)?;
        }
        self.todos.retain(|todo| !todo.completed);
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

    pub fn sort_todos(&mut self) {
        match self.sort_by {
            SortCriteria::Priority => self.todos.sort_by(|a, b| {
                let ordering = a.priority.unwrap().cmp(&b.priority.unwrap());
                if self.sort_asc {
                    ordering
                } else {
                    ordering.reverse()
                }
            }),
            SortCriteria::Completed => self.todos.sort_by(|a, b| {
                let ordering = a.completed.cmp(&b.completed);
                if self.sort_asc {
                    ordering
                } else {
                    ordering.reverse()
                }
            }),
            SortCriteria::CreatedDate => self.todos.sort_by(|a, b| {
                let ordering = a.created_at.cmp(&b.created_at);
                if self.sort_asc {
                    ordering
                } else {
                    ordering.reverse()
                }
            }),
        }
    }

    pub fn toggle_sort_direction(&mut self) {
        self.sort_asc = !self.sort_asc;
    }

    pub fn set_sort_criteria(&mut self, criteria: SortCriteria) {
        if self.sort_by == criteria {
            self.toggle_sort_direction();
            self.sort_todos();
        } else {
            self.sort_by = criteria;
            self.sort_asc = true;
            self.sort_todos();
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.todos.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.todos.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
    }
}
