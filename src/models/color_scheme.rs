use ratatui::style::{Color, Style};

pub struct ColorScheme {
    // background colors
    pub bg: Color,
    pub bg_dark: Color,
    pub bg_darker: Color,

    // foreground colors
    pub fg: Color,
    pub fg_light: Color,
    pub fg_dark: Color,

    // UI elements
    pub border: Color,
    pub accent: Color,
    pub error: Color,

    // task status colors
    pub completed: Color,
    pub pending: Color,
    pub in_progress: Color,

    // selection and interactive elements
    pub selection: Color,
    pub dropdown_bg: Color,

    // inactive elements
    pub inactive: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            // Background colors
            bg: Color::Rgb(31, 31, 31),        // #1F1F1F
            bg_dark: Color::Rgb(24, 24, 24),   // #181818
            bg_darker: Color::Rgb(32, 32, 32), // #202020

            // Foreground colors
            fg: Color::Rgb(204, 204, 204),       // #CCCCCC
            fg_light: Color::Rgb(255, 255, 255), // #FFFFFF
            fg_dark: Color::Rgb(157, 157, 157),  // #9D9D9D

            // UI elements
            border: Color::Rgb(43, 43, 43),  // #2B2B2B
            accent: Color::Rgb(0, 120, 212), // #0078D4
            error: Color::Rgb(248, 81, 73),  // #F85149

            // Task status colors
            completed: Color::Rgb(46, 160, 67),   // #2EA043
            pending: Color::Rgb(110, 118, 129),   // #6E7681
            in_progress: Color::Rgb(0, 120, 212), // #0078D4

            // Selection and interactive elements
            selection: Color::Rgb(21, 85, 194),  // #1555c2
            dropdown_bg: Color::Rgb(49, 49, 49), // #313131

            // Inactive elements
            inactive: Color::Rgb(110, 118, 129), // #6E7681
        }
    }
}

impl ColorScheme {
    /// Style for normal text
    pub fn text(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    /// Style for highlighted text
    pub fn highlighted_text(&self) -> Style {
        Style::default().fg(self.fg_light).bg(self.bg)
    }

    /// Style for secondary text
    pub fn secondary_text(&self) -> Style {
        Style::default().fg(self.fg_dark).bg(self.bg)
    }

    /// Style for borders
    pub fn border(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Style for selected borders
    pub fn selected_border(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Style for active titles
    pub fn title(&self) -> Style {
        Style::default().fg(self.fg_light)
    }

    /// Style for selected items
    pub fn selected(&self) -> Style {
        Style::default().fg(self.fg_light).bg(self.selection)
    }

    /// Style for completed tasks
    pub fn completed_task(&self) -> Style {
        Style::default().fg(self.completed)
    }

    /// Style for pending tasks
    pub fn pending_task(&self) -> Style {
        Style::default().fg(self.pending)
    }

    /// Style for in-progress tasks
    pub fn in_progress_task(&self) -> Style {
        Style::default().fg(self.in_progress)
    }

    /// Style for errors
    pub fn error(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Style for buttons
    pub fn button(&self) -> Style {
        Style::default().fg(self.bg).bg(self.accent)
    }

    /// Style for inactive buttons
    pub fn inactive_button(&self) -> Style {
        Style::default().fg(self.bg).bg(self.inactive)
    }

    /// Style for dropdowns
    pub fn dropdown(&self) -> Style {
        Style::default().fg(self.fg).bg(self.dropdown_bg)
    }
}
