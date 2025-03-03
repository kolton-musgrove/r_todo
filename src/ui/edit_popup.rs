use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::state::Mode;
use crate::models::todo::Priority;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectableField {
    Text,
    Priority,
}

pub struct EditingState {
    pub input_fields: InputFields,
    pub selected_field: Option<SelectableField>,
}

pub struct InputFields {
    pub text: String,
    pub priority: Option<Priority>,
}

pub fn render(frame: &mut Frame, area: Rect, mode: &Mode, editing_state: &EditingState) {
    match mode {
        Mode::Normal => return,
        Mode::Editing => {
            let is_create = editing_state.input_fields.text.is_empty();
            let title = if is_create {
                "Create ToDo"
            } else {
                "Edit ToDo"
            };

            let popup_area = centered_rect(60, 30, area);

            // clear the popup area
            frame.render_widget(Clear, popup_area);

            // create the outer popup block
            let popup = Block::default()
                .title(title)
                .style(Style::default())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            // create the layout for the input fields
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // text input
                    Constraint::Length(5), // priority input
                ])
                .split(popup_area);

            // render the outer block
            frame.render_widget(popup, popup_area);

            // render the text input
            let text_style = match editing_state.selected_field {
                Some(SelectableField::Text) => Style::default().yellow(),
                _ => Style::default(),
            };

            let text_input = Paragraph::new(editing_state.input_fields.text.as_str())
                .style(text_style)
                .block(
                    Block::default()
                        .title("Todo")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                );
            frame.render_widget(text_input, chunks[0]);

            // render the priority input
            let priorities = vec![Priority::High, Priority::Medium, Priority::Low];
            let priority_items: Vec<ListItem> = priorities
                .iter()
                .map(|p| {
                    let style = if editing_state.selected_field == Some(SelectableField::Priority)
                        && Some(*p) == editing_state.input_fields.priority
                    {
                        Style::default().yellow()
                    } else {
                        Style::default()
                    };

                    ListItem::new(format!("{}", p)).style(style)
                })
                .collect();

            let priority_list = List::new(priority_items)
                .block(
                    Block::default()
                        .title("Priority")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .highlight_style(Style::default().reversed());

            frame.render_widget(priority_list, chunks[1]);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
