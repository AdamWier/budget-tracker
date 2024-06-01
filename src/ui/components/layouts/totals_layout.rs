use std::{
    collections::BTreeMap,
    io::Write,
    rc::Rc,
    sync::{Arc, Mutex},
};

use ansi_to_tui::IntoText;
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
    chart_output: String,
}

impl TotalsLayout {
    pub fn init(
        budget_items: Vec<BudgetItem>,
        assigned_transactions_arc: &Arc<Mutex<Vec<AssignedTransaction>>>,
    ) -> Self {
        let assigned_transactions = Arc::clone(assigned_transactions_arc);
        let mut totals_layout = TotalsLayout {
            sections: 1,
            budget_items,
            assigned_transactions,
            chart_output: String::new(),
        };
        totals_layout.chart_stuff();
        totals_layout
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
    fn chart_stuff(&mut self) {
        use piechart::{Chart, Color as PColor, Data};

        let data = vec![
            Data {
                label: "Chocolate".into(),
                value: 4.0,
                color: Some(PColor::Blue.into()),
                fill: 'r',
            },
            Data {
                label: "Strawberry".into(),
                value: 2.0,
                color: Some(PColor::Red.into()),
                fill: 'r',
            },
            Data {
                label: "Vanilla".into(),
                value: 2.6,
                color: Some(PColor::Yellow.into()),
                fill: 'r',
            },
        ];
        let chart = Chart::new()
            .radius(9)
            .aspect_ratio(3)
            .legend(true)
            .draw_into(self, &data);
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
            .for_each(|(index, paragraph)| frame.render_widget(paragraph, layout[index]));

        let text = self.chart_output.into_text().unwrap();
        frame.render_widget(text, area);
    }
}

impl Write for TotalsLayout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.chart_output.push_str(&String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.chart_output = String::new();
        Ok(())
    }
}
