use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::models::todo::Todo;

pub fn render(frame: &mut Frame, area: Rect, todos: &[Todo]) {
    let todo_rows: Vec<Row> = todos
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

    let todos_table = Table::new(todo_rows, widths)
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

    frame.render_widget(todos_table, area);
}
