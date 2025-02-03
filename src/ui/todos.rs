use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, HighlightSpacing, Row, Table, TableState},
    Frame,
};

use crate::models::todo::Todo;

pub fn render(frame: &mut Frame, area: Rect, table_state: &mut TableState, todos: &[Todo]) {
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
        .style(Style::new().white())
        .header(
            Row::new(vec!["Completed", "Task", "Priority"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("ToDos"))
        .row_highlight_style(Style::new().yellow())
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(todos_table, area, table_state);
}
