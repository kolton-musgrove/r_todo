use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Cell, HighlightSpacing, Row, Table},
    Frame,
};

use crate::{
    app::state::{App, Mode, SortCriteria},
    models::color_scheme::ColorScheme,
};

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let colors = ColorScheme::default();

    app.sort_todos();

    let todo_rows: Vec<Row> = app
        .todos
        .iter()
        .map(|todo| {
            Row::new(vec![
                Cell::from(if todo.completed { "(✓)" } else { "( )" }),
                Cell::from(todo.text.clone()),
                Cell::from(format!("{}", todo.priority.unwrap())),
            ])
            .height(2)
            .style(Style::default().fg(if todo.completed {
                colors.completed
            } else {
                colors.fg
            }))
        })
        .collect();

    let widths = [
        Constraint::Percentage(10),
        Constraint::Percentage(65),
        Constraint::Percentage(25),
    ];

    let header_cells = vec![
        Cell::from({
            let mut status_header = String::from("Status");
            if app.sort_by == SortCriteria::Completed {
                status_header.push_str(if app.sort_asc { " ↑" } else { " ↓" })
            }
            status_header
        }),
        Cell::from("Task"),
        Cell::from({
            let mut priority_header = String::from("Prioriy");
            if app.sort_by == SortCriteria::Priority {
                priority_header.push_str(if app.sort_asc { " ↑" } else { " ↓" });
            }
            priority_header
        }),
    ];

    let todos_table = Table::new(todo_rows, widths)
        .column_spacing(1)
        .style(colors.fg)
        .header(
            Row::new(header_cells)
                .style(colors.title())
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .title("TODOs")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(if app.mode == Mode::Normal {
                    colors.selected_border()
                } else {
                    Style::default().bg(colors.bg)
                }),
        )
        .row_highlight_style(Style::new().fg(colors.selection))
        .highlight_symbol(">> ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(todos_table, area, &mut app.table_state);
}
