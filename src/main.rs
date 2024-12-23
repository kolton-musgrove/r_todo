use std::{
    fs,
    io::{self, stdout, Write},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, ClearType},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ToDo {
    title: String,
    completed: bool,
}

struct App {
    todos: Vec<ToDo>,
    selected: usize,
    input_mode: bool,
    current_input: String,
}

impl App {
    fn new() -> Self {
        Self {
            todos: Vec::new(),
            selected: 0,
            input_mode: false,
            current_input: String::new(),
        }
    }

    fn load_todos() -> Vec<ToDo> {
        match fs::read_to_string("todos.json") {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => Vec::new(),
        }
    }

    fn add_todo(&mut self, title: String) {
        self.todos.push(ToDo {
            title,
            completed: false,
        });
        self.save_todos().unwrap_or_default();
    }

    fn toggle_selected(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.completed = !todo.completed;
            self.save_todos().unwrap_or_default();
        }
    }

    fn delete_selected(&mut self) {
        if !self.todos.is_empty() {
            self.todos.remove(self.selected);
            if self.selected >= self.todos.len() && !self.todos.is_empty() {
                self.selected = self.todos.len() - 1;
            }
            self.save_todos().unwrap_or_default();
        }
    }

    fn save_todos(&self) -> io::Result<()> {
        let json = serde_json::to_string(&self.todos)?;
        fs::write("todos.json", json)
    }
}

fn main() -> io::Result<()> {
    // set up terminal
    let _ = terminal::enable_raw_mode();
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut app = App {
        todos: App::load_todos(),
        selected: 0,
        input_mode: false,
        current_input: String::new(),
    };

    loop {
        // clear screen
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print("ToDo list (q to quit, n for new, space to toggle, d to delete)\n\n")
        )?;

        // draw todos
        let mut current_line = 2u16;
        for (i, todo) in app.todos.iter().enumerate() {
            let prefix = if i == app.selected { "> " } else { " " };
            let checkmark = if todo.completed { "[x]" } else { "[ ] " };

            execute!(
                stdout,
                cursor::MoveTo(0, current_line),
                SetForegroundColor(if todo.completed {
                    Color::Green
                } else {
                    Color::White
                }),
                Print(format!("{}{} {}\n", prefix, checkmark, todo.title))
            )?;

            current_line += 1;
        }

        // draw input line if in input mode
        if app.input_mode {
            execute!(
                stdout,
                cursor::MoveTo(0, current_line + 1),
                SetForegroundColor(Color::Yellow),
                Print("New ToDo: "),
                Print(&app.current_input)
            )?;
        }

        stdout.flush()?;

        // handle keyboard input
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                // mode and toggle
                KeyCode::Char('q') if !app.input_mode => break,
                KeyCode::Char('n') if !app.input_mode => {
                    app.input_mode = true;
                }
                KeyCode::Char(' ') if !app.input_mode => app.toggle_selected(),
                KeyCode::Char('d') if !app.input_mode => app.delete_selected(),

                // input mode
                KeyCode::Char(c) if app.input_mode => app.current_input.push(c),
                KeyCode::Backspace if app.input_mode => {
                    let _ = app.current_input.pop();
                }
                KeyCode::Enter if app.input_mode => {
                    if !app.current_input.is_empty() {
                        app.add_todo(app.current_input.clone());
                        app.current_input.clear();
                        app.input_mode = false;
                    }
                }
                KeyCode::Esc if app.input_mode => {
                    app.current_input.clear();
                    app.input_mode = false;
                }

                // navigation keys
                KeyCode::Char('k') if !app.input_mode => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Char('j') if !app.input_mode => {
                    if !app.todos.is_empty() && app.selected < app.todos.len() - 1 {
                        app.selected += 1;
                    }
                }
                _ => {}
            }
        }
    }

    // cleanup
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
