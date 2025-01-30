use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::state::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
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
        frame.render_widget(info, area);
    }
}
