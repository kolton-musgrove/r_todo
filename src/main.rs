mod app;
mod db;
mod models;

use app::{
    state::{App, InputMode},
    ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize app with database
    let db_path = "todos.db";
    let mut app = match App::new(db_path) {
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

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(20);

    loop {
        // Draw the current state
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => app.show_help = !app.show_help,
                        KeyCode::Char('e') => {
                            if let Some(selected) = app.selected_index {
                                if let Some(todo) = app.todos.get(selected) {
                                    app.input = todo.text.clone();
                                    app.input_mode = InputMode::Editing;
                                }
                            }
                        }
                        KeyCode::Char('n') => {
                            app.input_mode = InputMode::Editing;
                            app.input.clear();
                        }
                        KeyCode::Char('d') => {
                            if let Some(selected) = app.selected_index {
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
                            if let Some(selected) = app.selected_index {
                                if let Err(e) = app.toggle_todo(selected) {
                                    app.set_error(format!("Failed to toggle todo: {}", e));
                                }
                            }
                        }
                        KeyCode::Char('k') => {
                            app.selected_index = match app.selected_index {
                                Some(i) => {
                                    if i > 0 {
                                        Some(i - 1)
                                    } else {
                                        Some(app.todos.len() - 1)
                                    }
                                }
                                None => Some(0),
                            }
                        }
                        KeyCode::Char('j') => {
                            app.selected_index = match app.selected_index {
                                Some(i) => {
                                    if i + 1 < app.todos.len() {
                                        Some(i + 1)
                                    } else {
                                        Some(0)
                                    }
                                }
                                None => Some(0),
                            };
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            if !app.input.is_empty() {
                                if let Some(selected) = app.selected_index {
                                    if let Err(e) =
                                        app.update_todo_text(selected, app.input.clone())
                                    {
                                        app.set_error(format!("Failed to update todo: {}", e));
                                    }
                                } else {
                                    if let Err(e) = app.add_todo(app.input.clone()) {
                                        app.set_error(format!("Failed to add todo: {}", e));
                                    }
                                }

                                app.input.clear();
                                app.input_mode = InputMode::Normal;
                                terminal.clear()?;
                            }
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input.clear();
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.check_error_timeout();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
