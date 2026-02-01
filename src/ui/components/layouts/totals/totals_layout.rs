use std::{ops::Mul, rc::Rc, str::FromStr};

use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDate, NaiveTime};
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

        let first_day_of_cycle = self
            .state
            .assigned_transactions
            .lock()
            .unwrap()
            .iter()
            .min_by(|a, b| {
                NaiveDate::from_str(&a.date)
                    .unwrap_or_default()
                    .cmp(&NaiveDate::from_str(&b.date).unwrap_or_default())
            })
            .and_then(|x| x.date.split("/").into_iter().collect_tuple())
            .map(|(day, month, year)| format!("{year}-{month}-{day}"))
            .context("Could not find date")
            .and_then(|x| NaiveDate::from_str(&x.trim()).context("Could not parse date"))?;

        let today = Local::now().date_naive();
        let time_since_start_of_cycle = today - first_day_of_cycle;
        let days_since_start_of_cycle = time_since_start_of_cycle.num_days() as f32;
        let number_of_days_in_cycle = 365.0 / 12.0;
        let one_week_of_the_cycle = 365.0 / 52.0;

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

                let chunked = chunk.into_iter().collect_vec();

                let total = chunked
                    .iter()
                    .fold(0.0, |accu, transaction| accu + transaction.amount)
                    .mul(-1.0);

                let amount_per_day = budget_amount / number_of_days_in_cycle;
                let max_to_date = amount_per_day * days_since_start_of_cycle;
                let projected_spending =
                    amount_per_day * (days_since_start_of_cycle + one_week_of_the_cycle);

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

        //Sections must be set before getting layout
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
