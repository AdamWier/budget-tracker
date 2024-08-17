use std::{
    collections::BTreeMap,
    ops::Mul,
    rc::Rc,
    sync::{Arc, Mutex},
};

use chrono::{Datelike, Local};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::{
    csv::models::{AssignedTransaction, BudgetItem, BudgetItemType},
    ui::components::{reusable::chart::RatatuiChart, Component},
    utils::get_days_in_current_month,
};

use super::total_information::TotalInformation;

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
    fn get_code_total_information(&self) -> Vec<TotalInformation> {
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
        let mut total_information: Vec<TotalInformation> = Vec::new();
        for (key, chunk) in assigned_transactions_by_code {
            let budget_item = budget_items_to_total.find(|x| x.code == *key).unwrap();
            let total = chunk
                .iter()
                .fold(0.0, |accu, transaction| accu + transaction.amount);

            let days_in_current_month = get_days_in_current_month() as f32;
            let current_day_of_month = Local::now().day() as f32;
            let max_to_date = budget_item.amount / days_in_current_month * current_day_of_month;
            let projected_spending =
                budget_item.amount / days_in_current_month * (current_day_of_month + 7.0);

            total_information.push(TotalInformation {
                budget_amount: budget_item.amount,
                label: budget_item.label.to_string(),
                total: total.mul(-1.0),
                max_to_date,
                projected_spending,
            })
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
        let mut total_paragraphs: Vec<Paragraph> = self
            .get_code_total_information()
            .into_iter()
            .map(|x| {
                Paragraph::new(Text::styled(
                    format!(
                        "{}: {}/{}\nMax to date: {}\nFor the coming week: {}",
                        x.label,
                        x.total,
                        x.budget_amount,
                        x.max_to_date,
                        (x.projected_spending - x.total).max(0.0)
                    ),
                    Style::default().fg(Color::Rgb(255, 176, 0)),
                ))
                .alignment(Alignment::Center)
            })
            .collect();

        let charts = self
            .get_code_total_information()
            .into_iter()
            .map(move |x| RatatuiChart::new(x.get_chart_data()));

        let total_sections = total_paragraphs.len() * 2;
        self.set_sections(total_sections as u16);

        let layout = self.get_layout(area);

        if total_paragraphs.is_empty() {
            total_paragraphs.push(
                Paragraph::new(Text::styled(
                    "No items to total",
                    Style::default().fg(Color::Rgb(255, 176, 0)),
                ))
                .alignment(Alignment::Center),
            );
        }

        total_paragraphs
            .iter()
            .enumerate()
            .for_each(|(index, paragraph)| frame.render_widget(paragraph, layout[index * 2]));
        charts
            .enumerate()
            .for_each(|(index, paragraph)| paragraph.draw_chart(frame, layout[(index * 2) + 1]))
    }
}
