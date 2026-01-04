use std::{ops::Mul, rc::Rc};

use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use itertools::Itertools;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::Paragraph,
    Frame,
};

use crate::{
    csv::models::{BudgetItem, BudgetItemType},
    ui::{
        components::{reusable::chart::RatatuiChart, Component, Tab},
        state::State,
    },
    utils::get_days_in_current_month,
};

use super::total_information::TotalInformation;

#[derive(Debug)]
pub struct TotalsLayout<'a> {
    sections: u16,
    state: &'a State,
}

impl TotalsLayout<'_> {
    pub fn init(state: &State) -> TotalsLayout<'_> {
        TotalsLayout { sections: 1, state }
    }
    fn get_code_total_information(&self) -> Result<Vec<TotalInformation>> {
        let budget_items_to_total = self
            .state
            .budget_items
            .iter()
            .filter(|x| x.setting == BudgetItemType::MULTI)
            .collect_vec();

        let codes_to_total = budget_items_to_total
            .iter()
            .map(|x| x.code.clone())
            .collect_vec();

        self.state
            .assigned_transactions
            .lock()
            .unwrap()
            .iter()
            .filter(|x| codes_to_total.contains(&x.code))
            .sorted_by(|a, b| a.code.cmp(&b.code))
            .chunk_by(|x| &x.code)
            .into_iter()
            .map(|(key, chunk)| {
                let BudgetItem {
                    amount: budget_amount,
                    label,
                    ..
                } = budget_items_to_total
                    .iter()
                    .find(|x| x.code == *key)
                    .with_context(|| format!("No budget item found for key {key}"))?;
                let total = chunk
                    .fold(0.0, |accu, transaction| accu + transaction.amount)
                    .mul(-1.0);

                let days_in_current_month = get_days_in_current_month()
                    .with_context(|| format!("Could not get days in current month"))?
                    as f32;
                let current_day_of_month = Local::now().day() as f32;
                let max_to_date = budget_amount / days_in_current_month * current_day_of_month;
                let projected_spending =
                    budget_amount / days_in_current_month * (current_day_of_month + 7.0);

                Ok(TotalInformation {
                    budget_amount: *budget_amount,
                    label: label.to_string(),
                    total,
                    max_to_date,
                    projected_spending,
                })
            })
            .collect()
    }
    fn set_sections(&mut self, sections: u16) {
        self.sections = std::cmp::max(sections, 1)
    }
}

impl Component<'_> for TotalsLayout<'_> {
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        let size_for_each = 100_u16.saturating_div(self.sections);
        let constraints = vec![Constraint::Percentage(size_for_each); self.sections.into()];
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let code_total_information = self.get_code_total_information().unwrap();

        let charts = code_total_information
            .iter()
            .map(move |x| RatatuiChart::new(x.get_chart_data()));

        let paragraphs_for_total = (!code_total_information.is_empty())
            .then(|| {
                code_total_information
                    .iter()
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
                    .collect_vec()
            })
            .unwrap_or(vec![Paragraph::new(Text::styled(
                "No items to total",
                Style::default().fg(Color::Rgb(255, 176, 0)),
            ))
            .alignment(Alignment::Center)]);

        //Sections must be set before getting layout => Change to pass sections as param
        self.set_sections((paragraphs_for_total.len() * 2) as u16);
        let layout = self.get_layout(area);
        paragraphs_for_total
            .iter()
            .enumerate()
            .for_each(|(index, paragraph)| frame.render_widget(paragraph, layout[index * 2]));
        charts
            .enumerate()
            .for_each(|(index, paragraph)| paragraph.draw_chart(frame, layout[(index * 2) + 1]))
    }
}

impl Tab for TotalsLayout<'_> {
    fn get_name(&self) -> String {
        "Totals".to_string()
    }
}
