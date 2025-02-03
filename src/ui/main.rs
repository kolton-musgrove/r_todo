use crate::app::state::App;
use crate::ui::edit_popup::render as render_popup;
use crate::ui::help_and_error::render as render_help_and_error;
use crate::ui::todos::render as render_todos;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
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
        .style(Style::default())
        .fg(Color::Cyan)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    render_todos(f, chunks[1], &mut app.table_state, &app.todos);
    render_help_and_error(f, chunks[2], &app);
    render_popup(f, f.area(), &app.mode, &app.editing_state)
}
