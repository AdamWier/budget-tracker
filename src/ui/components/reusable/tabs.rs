use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Tabs},
    Frame,
};

use crate::ui::components::{Component, Tab};

pub struct TabsManager<'a> {
    selected_tab_index: usize,
    tabs: Vec<Box<dyn Tab + 'a>>,
}

impl<'a> TabsManager<'a> {
    pub fn init(tabs: Vec<Box<dyn Tab + 'a>>) -> Self {
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
    pub fn get_tab_to_render(&mut self) -> &mut Box<dyn Tab + 'a> {
        &mut self.tabs[self.selected_tab_index]
    }
}

impl Component<'_> for TabsManager<'_> {
    fn handle_child_events(
        &mut self,
        event: &crossterm::event::Event,
    ) -> color_eyre::eyre::Result<()> {
        self.tabs[self.selected_tab_index].handle_events(event)
    }
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
        let tabs = Tabs::new(self.tabs.iter().map(|x| x.get_name()))
            .block(Block::bordered().title("Section"))
            .style(Style::default().fg(Color::Rgb(255, 176, 0)))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .select(self.selected_tab_index);

        frame.render_widget(tabs, area)
    }
}
