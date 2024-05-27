use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
};

use crate::ui::components::Component;

#[derive(Debug)]
pub struct Popup {
    text: String,
    color: Color,
}

impl Popup {
    pub fn init(text: String, color: Color) -> Self {
        Self { text, color }
    }
}

impl Component for Popup {
    fn render(&mut self, f: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let style = Style::new().fg(self.color);
        let text = Text::from(self.text.to_string()).style(style);
        let popup = Paragraph::new(text).alignment(Alignment::Center);
        f.render_widget(popup, area)
    }
}
