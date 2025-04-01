mod app;
mod db;
mod models;
mod ui;

use crate::models::todo::Priority;
use app::state::{App, Mode, SortCriteria};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use directories::ProjectDirs;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    fs, io,
    path::PathBuf,
    time::{Duration, Instant},
};
use ui::edit_popup::{EditingState, InputFields, SelectableField};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize app with database
    let db_path = get_database_path()?;
    let mut app = match App::new(db_path.to_str().unwrap()) {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return Err(e);
        }
    };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut last_selected = 0;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(20);

    // main loop
    loop {
        // Draw the current state
        terminal.draw(|f| ui::main::render(f, &mut app))?;

        // we use a timeout function to periodically check if a user event has occurred
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // poll for user events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => app.show_help = !app.show_help,
                        KeyCode::Char('n') => {
                            app.editing_state = EditingState {
                                input_fields: InputFields {
                                    text: String::new(),
                                    priority: Some(Priority::Medium),
                                },
                                selected_field: Some(SelectableField::Text),
                            };
                            last_selected = app.table_state.selected().unwrap_or(0);
                            app.table_state.select(None);
                            app.mode = Mode::Editing;
                        }
                        KeyCode::Char('e') => {
                            if let Some(selected) = app.table_state.selected() {
                                if let Some(todo) = app.todos.get(selected) {
                                    app.editing_state = EditingState {
                                        input_fields: InputFields {
                                            text: todo.text.clone(),
                                            priority: todo.priority,
                                        },
                                        selected_field: Some(SelectableField::Text),
                                    };
                                    app.mode = Mode::Editing;
                                }
                            }
                        }
                        KeyCode::Char('d') => {
                            if let Some(selected) = app.table_state.selected() {
                                if let Err(e) = app.delete_todo(selected) {
                                    app.set_error(format!("Failed to delete todo: {}", e));
                                }
                            }
                        }
                        KeyCode::Char('c') => {
                            if let Err(e) = app.clear_completed() {
                                app.set_error(format!("Failed to clear completed todos: {}", e));
                            }
                        }
                        KeyCode::Char(' ') => {
                            if let Some(selected) = app.table_state.selected() {
                                if let Err(e) = app.toggle_todo(selected) {
                                    app.set_error(format!("Failed to toggle todo: {}", e));
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.select_previous();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.select_next();
                        }
                        KeyCode::Char('p') => {
                            app.set_sort_criteria(SortCriteria::Priority);
                        }
                        KeyCode::Char('t') => {
                            app.set_sort_criteria(SortCriteria::CreatedDate);
                        }
                        KeyCode::Char('s') => {
                            app.set_sort_criteria(SortCriteria::Completed);
                        }
                        KeyCode::Char('r') => {
                            app.toggle_sort_direction();
                            app.sort_todos();
                        }
                        _ => {}
                    },
                    Mode::Editing => match key.code {
                        KeyCode::Char(c) => {
                            if let Some(SelectableField::Text) = app.editing_state.selected_field {
                                app.editing_state.input_fields.text.push(c);
                            } else if let Some(SelectableField::Priority) =
                                app.editing_state.selected_field
                            {
                                // allow the user to set the priority directly
                                if c == 'h' || c == '1' {
                                    app.editing_state.input_fields.priority = Some(Priority::High);
                                } else if c == 'm' || c == '2' {
                                    app.editing_state.input_fields.priority =
                                        Some(Priority::Medium);
                                } else if c == 'l' || c == '3' {
                                    app.editing_state.input_fields.priority = Some(Priority::Low);
                                }

                                // allow the user to naviagate through the priority options
                                if c == 'j' {
                                    let current = app
                                        .editing_state
                                        .input_fields
                                        .priority
                                        .unwrap_or(Priority::Medium);
                                    let next = match current {
                                        Priority::High => Priority::Medium,
                                        Priority::Medium => Priority::Low,
                                        Priority::Low => Priority::High,
                                    };
                                    app.editing_state.input_fields.priority = Some(next);
                                } else if c == 'k' {
                                    let current = app
                                        .editing_state
                                        .input_fields
                                        .priority
                                        .unwrap_or(Priority::Medium);
                                    let next = match current {
                                        Priority::High => Priority::Low,
                                        Priority::Medium => Priority::High,
                                        Priority::Low => Priority::Medium,
                                    };
                                    app.editing_state.input_fields.priority = Some(next);
                                }
                            } else {
                                continue;
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(SelectableField::Text) = app.editing_state.selected_field {
                                app.editing_state.input_fields.text.pop();
                            }
                        }
                        KeyCode::Tab => {
                            app.editing_state.selected_field =
                                match app.editing_state.selected_field {
                                    Some(SelectableField::Text) => Some(SelectableField::Priority),
                                    Some(SelectableField::Priority) => Some(SelectableField::Text),
                                    None => Some(SelectableField::Text),
                                };
                        }
                        KeyCode::Enter => {
                            if !app.editing_state.input_fields.text.is_empty() {
                                let priority = app
                                    .editing_state
                                    .input_fields
                                    .priority
                                    .unwrap_or(Priority::Medium);

                                if let Some(selected) = app.table_state.selected() {
                                    if let Err(e) = app.update_todo(
                                        selected,
                                        app.editing_state.input_fields.text.clone(),
                                        priority,
                                    ) {
                                        app.set_error(format!("Failed to update todo: {}", e));
                                    }
                                } else {
                                    if let Err(e) = app.add_todo(
                                        app.editing_state.input_fields.text.clone(),
                                        priority,
                                    ) {
                                        app.set_error(format!("Failed to add todo: {}", e));
                                    }

                                    app.table_state.select(Some(last_selected));
                                }

                                app.editing_state.input_fields.text.clear();
                                app.editing_state.input_fields.priority = None;
                                app.mode = Mode::Normal;
                                terminal.clear()?;
                            }
                        }
                        KeyCode::Esc => {
                            app.editing_state.input_fields.text.clear();
                            app.editing_state.input_fields.priority = None;
                            app.mode = Mode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }

        // check if an error has been shown for too long
        // and reset the timeout
        if last_tick.elapsed() >= tick_rate {
            app.check_error_timeout();
            last_tick = Instant::now();
        }
    }

    // cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// this project dir will be appropriate for respective OSs
fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let proj_dirs = ProjectDirs::from("com", "auxilia", "r_todo")
        .ok_or("Failed to determine project directories")?;

    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir)?;

    Ok(data_dir.join("todos.db"))
}
