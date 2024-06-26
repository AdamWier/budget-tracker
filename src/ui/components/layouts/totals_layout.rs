use std::{
    collections::BTreeMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::{
    csv::models::{AssignedTransaction, BudgetItem, BudgetItemType},
    ui::components::Component,
};

#[derive(Debug)]
pub struct TotalsLayout {
    sections: u16,
    budget_items: Vec<BudgetItem>,
    assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
}

impl TotalsLayout {
    pub fn init(
        budget_items: Vec<BudgetItem>,
        assigned_transactions_arc: &Arc<Mutex<Vec<AssignedTransaction>>>,
    ) -> Self {
        let assigned_transactions = Arc::clone(assigned_transactions_arc);
        TotalsLayout {
            sections: 1,
            budget_items,
            assigned_transactions,
        }
    }
    fn get_code_total_information(&self) -> Vec<(String, f32, f32)> {
        let mut budget_items_to_total = self
            .budget_items
            .iter()
            .filter(|x| x.setting == BudgetItemType::MULTI);
        let codes_to_total: Vec<String> = budget_items_to_total
            .clone()
            .map(|x| x.code.to_string())
            .collect();
        let assigned_transactions_binding = self.assigned_transactions.lock().unwrap();
        let assigned_transactions = assigned_transactions_binding
            .iter()
            .filter(|x| codes_to_total.contains(&x.code));
        let assigned_transactions_by_code = &assigned_transactions.into_iter().fold(
            BTreeMap::new(),
            |mut map: BTreeMap<String, Vec<&AssignedTransaction>>, x| {
                map.entry(x.code.to_string()).or_default().push(x);
                map
            },
        );
        let mut total_information: Vec<(String, f32, f32)> = Vec::new();
        for (key, chunk) in assigned_transactions_by_code {
            let budget_item = budget_items_to_total.find(|x| x.code == *key).unwrap();
            let total = chunk
                .iter()
                .fold(0.0, |accu, transaction| accu + transaction.amount);
            total_information.push((
                budget_item.label.to_string(),
                total.mul_add(-1.0, 0.0),
                budget_item.amount,
            ))
        }
        total_information
    }
    fn set_sections(&mut self, sections: u16) {
        self.sections = std::cmp::max(sections, 1)
    }
}

impl Component for TotalsLayout {
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        let size_for_each = 100_u16.saturating_div(self.sections);
        let mut constraints: Vec<Constraint> = Vec::new();
        for _constraint in 0..self.sections {
            constraints.push(Constraint::Percentage(size_for_each))
        }
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let mut paragraphs: Vec<Paragraph> = self
            .get_code_total_information()
            .into_iter()
            .map(|(code, total, amount)| {
                Paragraph::new(Text::styled(
                    format!("{}: {}/{}", code, total, amount),
                    Style::default().fg(Color::Rgb(255, 176, 0)),
                ))
                .alignment(Alignment::Center)
            })
            .collect();

        self.set_sections(paragraphs.len() as u16);

        let layout = self.get_layout(area);

        if paragraphs.is_empty() {
            paragraphs.push(
                Paragraph::new(Text::styled(
                    "No items to total",
                    Style::default().fg(Color::Rgb(255, 176, 0)),
                ))
                .alignment(Alignment::Center),
            );
        }

        paragraphs
            .iter()
            .enumerate()
            .for_each(|(index, paragraph)| frame.render_widget(paragraph, layout[index]))
    }
}
