use std::rc::Rc;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    csv::models::{BudgetItem, Transaction},
    ui::components::{reusable::scrollable_list::ScrollableList, Component},
};

#[derive(Debug)]
pub struct ActivityAreaLayout {
    transaction_list: ScrollableList,
    budget_list: ScrollableList,
}

impl ActivityAreaLayout {
    pub fn init(transactions: Vec<Transaction>, budget_items: Vec<BudgetItem>) -> Self {
        let transaction_list_items: Vec<String> = transactions
            .iter()
            .map(|x| format!("{} - {} - {}", x.date, x.label, x.amount))
            .collect();
        let budget_list_items: Vec<String> = budget_items
            .iter()
            .map(|x| format!("{} - {}", x.label, x.amount))
            .collect();
        ActivityAreaLayout {
            transaction_list: ScrollableList::init(
                transaction_list_items,
                KeyCode::Up,
                KeyCode::Down,
            ),
            budget_list: ScrollableList::init(
                budget_list_items,
                KeyCode::Char('8'),
                KeyCode::Char('2'),
            ),
        }
    }
}

impl Component for ActivityAreaLayout {
    fn handle_child_events(&mut self, event: &Event) -> color_eyre::eyre::Result<()> {
        self.transaction_list.handle_events(event)?;
        self.budget_list.handle_events(event)
    }
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let [transaction_chunk, budget_chunk] = *self.get_layout(area) else {
            panic!()
        };
        self.budget_list.render(frame, budget_chunk);
        self.transaction_list.render(frame, transaction_chunk);
    }
}
