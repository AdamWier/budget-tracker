use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use crossterm::event::Event;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{
    balance_layout::BalanceLayout, totals_layout::TotalsLayout,
    transaction_assignment_layout::TransactionAssignmentLayout,
};
use crate::{
    csv::models::{AssignedTransaction, BudgetItem, ParseResult},
    ui::components::{reusable::tabs::TabsManager, Component},
};

#[derive(Debug)]
pub struct MainLayout {
    transaction_assignment_layout: TransactionAssignmentLayout,
    totals_layout: TotalsLayout,
    tabs_manager: TabsManager,
    balance_layout: BalanceLayout,
}

impl MainLayout {
    pub fn init(
        parse_result: ParseResult,
        budget_items: Vec<BudgetItem>,
        assigned_transactions_arc: Arc<Mutex<Vec<AssignedTransaction>>>,
    ) -> Self {
        let tabs = ["Sorter", "Totals"];

        Self {
            transaction_assignment_layout: TransactionAssignmentLayout::init(
                parse_result.transactions,
                budget_items.clone(),
                &assigned_transactions_arc,
            ),
            totals_layout: TotalsLayout::init(budget_items.clone(), &assigned_transactions_arc),
            tabs_manager: TabsManager::init(tabs.map(String::from).to_vec()),
            balance_layout: BalanceLayout::init(
                budget_items.clone(),
                &assigned_transactions_arc,
                parse_result.balance,
            ),
        }
    }
    fn get_footer_layout(&self, parent_chunk: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(parent_chunk)
    }
}

impl Component for MainLayout {
    fn handle_child_events(&mut self, event: &Event) -> color_eyre::eyre::Result<()> {
        self.transaction_assignment_layout.handle_events(event)?;
        self.tabs_manager.handle_events(event)
    }
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        let title = Paragraph::new(Text::styled(
            "World's Best Budget Manager",
            Style::default().fg(Color::Rgb(255, 176, 0)),
        ))
        .alignment(Alignment::Center)
        .block(block.clone());

        let [title_chunk, transaction_chunk, footer_chunk] = *self.get_layout(area) else {
            panic!()
        };
        let [tabs_chunk, balance_chunk] = *self.get_footer_layout(footer_chunk) else {
            panic!()
        };

        frame.render_widget(title, title_chunk);
        match self.tabs_manager.selected_tab_index {
            0 => self
                .transaction_assignment_layout
                .render(frame, transaction_chunk),
            1 => self.totals_layout.render(frame, transaction_chunk),
            _ => panic!(),
        }
        self.tabs_manager.render(frame, tabs_chunk);
        self.balance_layout.render(frame, balance_chunk);
    }
}
