use std::rc::Rc;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::csv::models::Transaction;

use super::{scrollable_list::ScrollableList, Component};

#[derive(Debug)]
pub struct ActivityAreaLayout {
    transaction_list: ScrollableList,
}

impl ActivityAreaLayout {
    pub fn init(transactions: Vec<Transaction>) -> Self {
        let list_items: Vec<String> = transactions
            .iter()
            .map(|x| format!("{} - {} - {}", x.date, x.label, x.amount))
            .collect();
        ActivityAreaLayout {
            transaction_list: ScrollableList::init(list_items, KeyCode::Up, KeyCode::Down),
        }
    }
}

impl Component for ActivityAreaLayout {
    fn handle_child_events(&mut self, event: &Event) -> color_eyre::eyre::Result<()> {
        self.transaction_list.handle_events(event)
    }
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let [transaction_chunk, budget_chunk] = *self.get_layout(area) else {
            panic!()
        };
        self.transaction_list.render(frame, transaction_chunk);
    }
}
