use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    csv::models::{AssignedTransaction, BudgetItem, BudgetItemType},
    ui::components::Component,
};

#[derive(Debug)]
pub struct BalanceLayout {
    budget_items: Vec<BudgetItem>,
    assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
    balance: f32,
}

impl BalanceLayout {
    pub fn init(
        budget_items: Vec<BudgetItem>,
        assigned_transactions_arc: &Arc<Mutex<Vec<AssignedTransaction>>>,
        balance: f32,
    ) -> Self {
        let assigned_transactions = Arc::clone(assigned_transactions_arc);

        Self {
            balance,
            assigned_transactions,
            budget_items: budget_items.clone(),
        }
    }
    fn get_projected_balance(&self) -> f32 {
        let budget_items_to_total: Vec<&BudgetItem> = self
            .budget_items
            .iter()
            .filter(|x| x.setting == BudgetItemType::SING)
            .collect();
        let assigned_transactions_binding = self.assigned_transactions.lock().unwrap();
        let mut assigned_transactions_codes: Vec<String> = assigned_transactions_binding
            .iter()
            .map(|x| x.code.to_string())
            .collect();
        assigned_transactions_codes.sort();
        assigned_transactions_codes.dedup();

        let total_pending = budget_items_to_total
            .into_iter()
            .filter(|x| !assigned_transactions_codes.contains(&x.code))
            .fold(0.0, |accu, item| accu + item.amount);
        self.balance - total_pending
    }
}

impl Component for BalanceLayout {
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        let current_balance_paragraph = Paragraph::new(Text::styled(
            format!("Balance: {}", self.balance),
            Style::default().fg(Color::Rgb(255, 176, 0)),
        ))
        .alignment(Alignment::Center)
        .block(block.clone());

        let projected_balance = self.get_projected_balance();

        let projected_balance_paragraph = Paragraph::new(Text::styled(
            format!("Projected balance: {}", projected_balance),
            Style::default().fg(Color::Rgb(255, 176, 0)),
        ))
        .alignment(Alignment::Center)
        .block(block.clone());

        let [projected_balance_chunk, current_balance_chunk] = *self.get_layout(area) else {
            panic!()
        };

        frame.render_widget(current_balance_paragraph, current_balance_chunk);
        frame.render_widget(projected_balance_paragraph, projected_balance_chunk);
    }
}
