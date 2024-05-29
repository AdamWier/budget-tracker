use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Tabs},
    Frame,
};

use crate::ui::components::Component;

#[derive(Debug)]
pub struct TabsManager {
    pub selected_tab_index: usize,
    tabs: Vec<String>,
}

impl TabsManager {
    pub fn init(tabs: Vec<String>) -> Self {
        TabsManager {
            selected_tab_index: 0,
            tabs,
        }
    }
    fn increase_tab_index(&mut self) -> color_eyre::eyre::Result<()> {
        self.selected_tab_index = std::cmp::min(
            self.selected_tab_index.saturating_add(1),
            self.tabs.len().saturating_sub(1),
        );
        Ok(())
    }
    fn decrease_tab_index(&mut self) -> color_eyre::eyre::Result<()> {
        self.selected_tab_index = std::cmp::max(self.selected_tab_index.saturating_sub(1), 0);
        Ok(())
    }
}

impl Component for TabsManager {
    fn handle_key_events(
        &mut self,
        key: &crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<()> {
        match key.code {
            KeyCode::Right => self.increase_tab_index(),
            KeyCode::Left => self.decrease_tab_index(),
            _ => Ok(()),
        }
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let tabs = Tabs::new(self.tabs.clone())
            .block(Block::bordered().title("Section"))
            .style(Style::default().fg(Color::Rgb(255, 176, 0)))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .select(self.selected_tab_index);

        frame.render_widget(tabs, area)
    }
}
