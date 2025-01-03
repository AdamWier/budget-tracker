use std::rc::Rc;

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    csv::{
        models::{list_item::ListItem, BudgetItemType},
        persister::persist_association,
    },
    ui::{
        components::{reusable::scrollable_list::ScrollableList, Component},
        state::State,
    },
};

#[derive(Debug)]
pub struct TransactionAssignmentLayout<'a> {
    transaction_list: ScrollableList,
    budget_list: ScrollableList,
    state: &'a State,
}

impl TransactionAssignmentLayout<'_> {
    pub fn init(state: &'_ State) -> TransactionAssignmentLayout<'_> {
        let mut boxed_transactions = Vec::new();
        for item in state.transactions.clone().into_iter() {
            boxed_transactions.push(Box::new(item) as Box<dyn ListItem>)
        }

        TransactionAssignmentLayout {
            transaction_list: ScrollableList::init(boxed_transactions, KeyCode::Up, KeyCode::Down),
            budget_list: ScrollableList::init(Vec::new(), KeyCode::Char('8'), KeyCode::Char('2')),
            state,
        }
    }
    fn update_budget_list_items(&mut self) {
        let assigned_codes: Vec<String> = self
            .state
            .assigned_transactions
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.code.to_string())
            .collect();
        let budget_items_left =
            self.state.budget_items.clone().into_iter().filter(|x| {
                x.setting == BudgetItemType::MULTI || !assigned_codes.contains(&x.code)
            });

        let mut boxed_budget_items = Vec::new();
        for item in budget_items_left.into_iter().filter(|x| x.code != "SAL") {
            boxed_budget_items.push(Box::new(item) as Box<dyn ListItem>)
        }

        self.budget_list.update_list_items(boxed_budget_items)
    }
    fn assign_item(&mut self) {
        let transaction_item = self.transaction_list.get_selected_item();
        let budget_item = self.budget_list.get_selected_item();
        if budget_item.is_none() || transaction_item.is_none() {
            return;
        }
        persist_association(budget_item.unwrap(), transaction_item.unwrap()).unwrap();
        self.transaction_list.remove_selected_item();
    }
    fn handle_enter_key(&mut self) {
        self.assign_item()
    }
}

#[allow(clippy::single_match)]
impl Component<'_> for TransactionAssignmentLayout<'_> {
    fn handle_key_events(&mut self, key_event: &KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Enter => self.handle_enter_key(),
            _ => {}
        }
        Ok(())
    }
    fn handle_child_events(&mut self, event: &Event) -> Result<()> {
        self.transaction_list.handle_events(event)?;
        self.budget_list.handle_events(event)?;
        Ok(())
    }
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        self.update_budget_list_items();
        let [transaction_chunk, budget_chunk] = *self.get_layout(area) else {
            panic!()
        };
        self.budget_list.render(frame, budget_chunk);
        self.transaction_list.render(frame, transaction_chunk);
    }
}
