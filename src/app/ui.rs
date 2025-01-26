use crate::app::state::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let constraints = if app.show_help || app.error_message.is_some() {
        vec![
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(11),
            Constraint::Length(3),
        ]
    } else {
        vec![
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.area());

    let title = Paragraph::new(Text::from("r_Todo"))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let todos: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let style = if todo.completed {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if Some(i) == app.selected_index {
                "> "
            } else {
                " "
            };

            let line = Line::from(vec![
                Span::raw(prefix),
                Span::styled(
                    format!("[{}] {}", if todo.completed { "x" } else { " " }, todo.text),
                    style,
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let todos_list = List::new(todos).block(Block::default().borders(Borders::ALL).title("Tasks"));
    f.render_widget(todos_list, chunks[1]);

    let info_text = if app.show_help {
        app.get_help_text()
    } else if let Some(error) = &app.error_message {
        error.iter().map(|e| Line::from(e.as_str())).collect()
    } else {
        vec![Line::from(String::new())]
    };

    let info_style = if app.show_help {
        Style::default()
    } else {
        Style::default().fg(Color::Red)
    };

    let info_title = if app.show_help { "Help" } else { "Error" };

    if app.show_help || app.error_message.is_some() {
        let info = Paragraph::new(info_text)
            .style(info_style)
            .block(Block::default().borders(Borders::ALL).title(info_title));
        f.render_widget(info, chunks[2]);
    }

    let input = Paragraph::new(Text::from(app.input.as_str()))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));

    if app.show_help || app.error_message.is_some() {
        f.render_widget(input, chunks[3]);
    } else {
        f.render_widget(input, chunks[2]);
    }
}
