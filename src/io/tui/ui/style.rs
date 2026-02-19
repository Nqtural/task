use ratatui::style::Color;

pub struct UiStyle {
    pub border_color: Color,
    pub highlight_symbol: String,
    pub selected_background: Color,
    pub selected_foreground: Color,
}

impl UiStyle {
    pub fn new(active: bool) -> Self {
        if active {
            Self {
                border_color: Color::Green,
                highlight_symbol: String::from(" > "),
                selected_background: Color::Reset,
                selected_foreground: Color::Green,
            }
        } else {
            Self {
                border_color: Color::Reset,
                highlight_symbol: String::from(" > "),
                selected_background: Color::Reset,
                selected_foreground: Color::Blue,
            }
        }
    }
}
