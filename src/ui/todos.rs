use crate::app::state::App;
use crate::ui::edit_popup::render as render_popup;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
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

    let todo_rows: Vec<Row> = app
        .todos
        .iter()
        .map(|todo| {
            Row::new(vec![
                Cell::from(todo.text.clone()),
                Cell::from(format!("{}", todo.priority.unwrap())),
                Cell::from(todo.completed.to_string()),
            ])
            .height(2)
        })
        .collect();

    let widths = [
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
    ];

    let todos = Table::new(todo_rows, widths)
        .column_spacing(1)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Task", "Priority", "Completed"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("ToDos"))
        .row_highlight_style(Style::default().fg(Color::Yellow))
        .column_highlight_style(Style::default().fg(Color::Yellow))
        .cell_highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">");

    f.render_widget(todos, chunks[1]);

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

    render_popup(f, f.area(), &app.mode, &app.editing_state)
}
