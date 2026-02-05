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
    components::{
        layouts::cycles_layout::CyclesLayout, reusable::tabs::TabsManager, Component, Tab,
    },
    state::State,
};

pub struct MainLayout<'a> {
    tabs_manager: TabsManager<'a>,
    balance_layout: BalanceLayout<'a>,
    state: &'a State,
}

impl<'a> MainLayout<'a> {
    pub fn init(state: &'a State) -> MainLayout<'a> {
        let tabs = vec![
            Box::new(CyclesLayout::init(state)) as Box<dyn Tab>,
            Box::new(TransactionAssignmentLayout::init(state)) as Box<dyn Tab>,
            Box::new(TotalsLayout::init(state)) as Box<dyn Tab>,
        ];

        Self {
            tabs_manager: TabsManager::init(tabs),
            balance_layout: BalanceLayout::init(state),
            state,
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
            format!(
                "World's Best Budget Manager (Current file: {})",
                self.state.current_file.lock().unwrap().list_label
            ),
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
        self.tabs_manager
            .get_tab_to_render()
            .render(frame, transaction_chunk);
        self.tabs_manager.render(frame, tabs_chunk);
        self.balance_layout.render(frame, balance_chunk);
    }
}
