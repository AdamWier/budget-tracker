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
    balance_layout::BalanceLayout, totals::TotalsLayout,
    transaction_assignment_layout::TransactionAssignmentLayout,
};
use crate::ui::{
    components::{reusable::tabs::TabsManager, Component},
    state::State,
};

#[derive(Debug)]
pub struct MainLayout<'a> {
    transaction_assignment_layout: TransactionAssignmentLayout<'a>,
    totals_layout: TotalsLayout<'a>,
    tabs_manager: TabsManager,
    balance_layout: BalanceLayout<'a>,
}

impl<'a> MainLayout<'a> {
    pub fn init(state: &'a State) -> MainLayout<'a> {
        let tabs = ["Sorter", "Totals"];

        Self {
            transaction_assignment_layout: TransactionAssignmentLayout::init(state),
            totals_layout: TotalsLayout::init(state),
            tabs_manager: TabsManager::init(tabs.map(String::from).to_vec()),
            balance_layout: BalanceLayout::init(state),
        }
    }
    fn get_footer_layout(&self, parent_chunk: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(parent_chunk)
    }
}

impl Component<'_> for MainLayout<'_> {
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
