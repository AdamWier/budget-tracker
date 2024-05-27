use std::rc::Rc;

use crossterm::event::Event;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{
    totals_layout::TotalsLayout, transaction_assignment_layout::TransactionAssignmentLayout,
};
use crate::{
    csv::models::{BudgetItem, ParseResult},
    ui::components::{reusable::tabs::TabsManager, Component},
};

#[derive(Debug)]
pub struct MainLayout {
    transaction_assignment_layout: TransactionAssignmentLayout,
    totals_layout: TotalsLayout,
    balance: f32,
    tabs_manager: TabsManager,
}

impl MainLayout {
    pub fn init(parse_result: ParseResult, budget_items: Vec<BudgetItem>) -> Self {
        let tabs = ["Sorter", "Totals"];

        MainLayout {
            transaction_assignment_layout: TransactionAssignmentLayout::init(
                parse_result.transactions,
                budget_items,
            ),
            totals_layout: TotalsLayout::init(),
            balance: parse_result.balance,
            tabs_manager: TabsManager::init(tabs.to_vec().into_iter().map(String::from).collect()),
        }
    }
    fn get_footer_layout(&self, parent_chunk: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
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

        let balance = Paragraph::new(Text::styled(
            format!("Balance: {}", self.balance),
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

        if self.tabs_manager.selected_tab_index == 0 {
            self.transaction_assignment_layout
                .render(frame, transaction_chunk);
        }
        if self.tabs_manager.selected_tab_index == 1 {
            self.totals_layout.render(frame, transaction_chunk);
        }

        frame.render_widget(title, title_chunk);
        // page_to_render.render(frame, transaction_chunk);
        self.tabs_manager.render(frame, tabs_chunk);
        frame.render_widget(balance, balance_chunk)
    }
}
