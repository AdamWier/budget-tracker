use std::cmp;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

use crate::{csv::models::list_item::ListItem, ui::components::Component};

#[derive(Debug)]
pub struct ScrollableList {
    list_items: Vec<Box<dyn ListItem>>,
    list_state: ListState,
    list_screen_lines: usize,
    down_button: KeyCode,
    up_button: KeyCode,
}

impl ScrollableList {
    pub fn init(
        list_items: Vec<Box<dyn ListItem>>,
        up_button: KeyCode,
        down_button: KeyCode,
    ) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_items,
            up_button,
            down_button,
            list_state,
            list_screen_lines: 0,
        }
    }
    pub fn get_selected_item(&self) -> Option<&dyn ListItem> {
        self.list_state
            .selected()
            .and_then(|x| Some(self.list_items.get(x)?.as_ref()))
    }
    fn scroll_down(&mut self) -> Result<()> {
        let transaction_list_max = self.list_items.len().saturating_sub(self.list_screen_lines);
        let new_offset = self.list_state.offset() + 1usize;
        *self.list_state.offset_mut() = if new_offset <= transaction_list_max {
            new_offset
        } else {
            transaction_list_max
        };

        self.select_next()?;

        Ok(())
    }
    fn scroll_up(&mut self) -> Result<()> {
        let new_offset = if self.list_state.offset() == 0usize {
            0
        } else {
            self.list_state.offset() - 1usize
        };
        *self.list_state.offset_mut() = new_offset;

        self.select_previous()?;
        Ok(())
    }
    fn select_next(&mut self) -> Result<()> {
        let new_selected_index = self.list_state.selected().unwrap_or(0).saturating_add(1);
        let next_select = cmp::min(new_selected_index, self.list_items.len() - 1);
        self.list_state.select(Some(next_select));

        Ok(())
    }
    fn select_previous(&mut self) -> Result<()> {
        let new_selected_index = self.list_state.selected().unwrap_or(0).saturating_sub(1);
        let previous_select = cmp::max(new_selected_index, 0);
        self.list_state.select(Some(previous_select));

        Ok(())
    }
    pub fn remove_selected_item(&mut self) {
        let selected_item = self.get_selected_item().expect("Item was not found");
        let index = self
            .list_items
            .iter()
            .position(|x| x.get_savable_value() == selected_item.get_savable_value())
            .expect("Item was not found");
        self.list_items.remove(index);
    }
    pub fn update_list_items(&mut self, list_items: Vec<Box<dyn ListItem>>) {
        self.list_items = list_items;
    }
}

impl Component<'_> for ScrollableList {
    fn handle_key_events(&mut self, key: &crossterm::event::KeyEvent) -> Result<()> {
        let code = key.code;
        match code {
            _ if code == self.down_button => self.scroll_down(),
            _ if code == self.up_button => self.scroll_up(),
            _ => Ok(()),
        }
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.list_screen_lines = area.rows().count().saturating_sub(4);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        let [chunk] = *self.get_layout(area) else {
            panic!()
        };

        if self.list_items.is_empty() {
            let paragraph = Paragraph::new("The list is empty")
                .style(Style::default().fg(Color::Rgb(255, 176, 0)))
                .alignment(Alignment::Center)
                .block(block);
            frame.render_widget(paragraph, chunk)
        } else {
            let list = List::new(self.list_items.iter().map(|item| item.get_list_label()))
                .style(Style::default().fg(Color::Rgb(255, 176, 0)))
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

            frame.render_stateful_widget(list, chunk, &mut self.list_state);
        }
    }
}
