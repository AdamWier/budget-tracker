use std::rc::Rc;

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    csv::{
        models::{list_item::ListItem, BudgetItem, Transaction},
        persister::persist_association,
    },
    ui::components::{reusable::scrollable_list::ScrollableList, Component},
};

#[derive(Debug)]
pub struct ActivityAreaLayout {
    transaction_list: ScrollableList,
    budget_list: ScrollableList,
}

impl ActivityAreaLayout {
    pub fn init(transactions: Vec<Transaction>, budget_items: Vec<BudgetItem>) -> Self {
        let mut boxed_transactions = Vec::new();
        for item in transactions.into_iter() {
            boxed_transactions.push(Box::new(item) as Box<dyn ListItem>)
        }

        let mut boxed_budget_items = Vec::new();
        for item in budget_items.into_iter() {
            boxed_budget_items.push(Box::new(item) as Box<dyn ListItem>)
        }
        ActivityAreaLayout {
            transaction_list: ScrollableList::init(boxed_transactions, KeyCode::Up, KeyCode::Down),
            budget_list: ScrollableList::init(
                boxed_budget_items,
                KeyCode::Char('8'),
                KeyCode::Char('2'),
            ),
        }
    }
    fn assign_item(&self) {
        let transaction_item = self.transaction_list.get_selected_item();
        let budget_item = self.budget_list.get_selected_item();
        persist_association(budget_item, transaction_item);
    }
}

impl Component for ActivityAreaLayout {
    fn handle_key_events(&mut self, key_event: &KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Enter => self.assign_item(),
            _ => {}
        }
        Ok(())
    }
    fn handle_child_events(&mut self, event: &Event) -> Result<()> {
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
