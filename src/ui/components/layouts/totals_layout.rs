use std::rc::Rc;

use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::{
    csv::{
        models::{AssignedTransaction, BudgetItem, BudgetItemType},
        parsers::assigned_transactions::parse_assigned_transactions_csv,
    },
    ui::components::Component,
};

#[derive(Debug)]
pub struct TotalsLayout {
    sections: u16,
    budget_items: Vec<BudgetItem>,
}

impl TotalsLayout {
    pub fn init(budget_items: Vec<BudgetItem>) -> Self {
        TotalsLayout {
            sections: 1,
            budget_items,
        }
    }
    fn get_code_total_pairs(&self) -> Vec<(String, f32)> {
        let codes_to_total: Vec<String> = self
            .budget_items
            .clone()
            .into_iter()
            .filter(|x| x.setting == BudgetItemType::MULTI)
            .map(|x| x.code)
            .collect();
        let assigned_transactions = parse_assigned_transactions_csv("assigned_transactions.csv")
            .into_iter()
            .filter(|x| codes_to_total.contains(&x.code));
        let assigned_transactions_by_code = &assigned_transactions
            .into_iter()
            .chunk_by(|x| x.code.clone());
        let mut code_total_pairs: Vec<(String, f32)> = Vec::new();
        for (key, chunk) in assigned_transactions_by_code {
            let total = chunk
                .collect::<Vec<AssignedTransaction>>()
                .into_iter()
                .fold(0f32, |accu, transaction| accu + transaction.amount);
            code_total_pairs.push((key, total))
        }
        code_total_pairs
    }
}

impl<'a> Component for TotalsLayout {
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
        let paragraphs = self
            .get_code_total_pairs()
            .iter()
            .map(|(code, total)| {
                Paragraph::new(Text::styled(
                    format!("{}: {}", code, total),
                    Style::default().fg(Color::Rgb(255, 176, 0)),
                ))
                .alignment(Alignment::Center)
            })
            .collect::<Vec<Paragraph>>();

        self.sections = paragraphs.len() as u16;

        let layout = self.get_layout(area);

        paragraphs
            .iter()
            .enumerate()
            .for_each(|(index, paragraph)| frame.render_widget(paragraph, layout[index]))
    }
}
