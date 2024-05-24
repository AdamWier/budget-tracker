use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

use crate::ui::components::Component;

#[derive(Debug)]
pub struct ScrollableList {
    list_items: Vec<String>,
    list_state: ListState,
    list_screen_lines: usize,
    down_button: KeyCode,
    up_button: KeyCode,
}

impl ScrollableList {
    pub fn init(list_items: Vec<String>, up_button: KeyCode, down_button: KeyCode) -> Self {
        Self {
            list_items,
            up_button,
            down_button,
            list_state: ListState::default(),
            list_screen_lines: 0,
        }
    }
    pub fn scroll_down(&mut self) {
        let transaction_list_max = self.list_items.len() - self.list_screen_lines;
        let new_offset = *self.list_state.offset_mut() + 1usize;
        *self.list_state.offset_mut() = if new_offset <= transaction_list_max {
            new_offset
        } else {
            transaction_list_max
        };
    }
    pub fn scroll_up(&mut self) {
        let new_offset = if self.list_state.offset() == 0usize {
            0
        } else {
            self.list_state.offset() - 1usize
        };
        *self.list_state.offset_mut() = new_offset;
    }
}

impl Component for ScrollableList {
    fn handle_key_events(&mut self, key: &crossterm::event::KeyEvent) -> () {
        let code = key.code;
        match code {
            _ if code == self.down_button => self.scroll_down(),
            _ if code == self.up_button => self.scroll_up(),
            _ => {}
        }
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        let [chunk] = *self.get_layout(area) else {
            panic!()
        };

        let list = List::new(self.list_items.to_owned())
            .style(Style::default().fg(Color::Rgb(255, 176, 0)))
            .block(block);

        frame.render_stateful_widget(list, chunk, &mut self.list_state);
    }
}
