use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::models::todo::Todo;

pub fn render(frame: &mut Frame, area: Rect, todos: &[Todo]) {
    let todo_rows: Vec<Row> = todos
        .iter()
        .map(|todo| {
            Row::new(vec![
                Cell::from(if todo.completed { "[✓]" } else { "[ ]" }),
                Cell::from(todo.text.clone()),
                Cell::from(format!("{}", todo.priority.unwrap())),
            ])
            .height(2)
        })
        .collect();

    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(50),
        Constraint::Percentage(40),
    ];

    let todos_table = Table::new(todo_rows, widths)
        .column_spacing(1)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Completed", "Task", "Priority"])
                .style(Style::new().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("ToDos"))
        .row_highlight_style(Style::default().fg(Color::Yellow))
        .column_highlight_style(Style::default().fg(Color::Yellow))
        .cell_highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">");

    frame.render_widget(todos_table, area);
}
