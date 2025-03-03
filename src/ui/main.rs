use crate::app::state::App;
use crate::ui::edit_popup::render as render_popup;
use crate::ui::help_and_error::render as render_help_and_error;
use crate::ui::todos::render as render_todos;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Padding, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let constraints = if app.show_help || app.error_message.is_some() {
        vec![
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(11),
        ]
    } else {
        vec![Constraint::Length(3), Constraint::Min(10)]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(3)
        .constraints(constraints)
        .split(f.area());

    let title = Paragraph::new(Text::from("r_Todo"))
        .style(Style::default())
        .fg(Color::Cyan)
        .alignment(Alignment::Center)
        .block(Block::default().padding(Padding::top(1)));
    f.render_widget(title, chunks[0]);

    render_todos(f, chunks[1], &mut app.table_state, &app.todos);

    if app.show_help || app.error_message.is_some() {
        render_help_and_error(f, chunks[2], &app);
    }

    render_popup(f, f.area(), &app.mode, &app.editing_state)
}
