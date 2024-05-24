use std::rc::Rc;

use crossterm::event::Event;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{activity_area_layout::ActivityAreaLayout, Component};
use crate::csv::models::ParseResult;

#[derive(Debug)]
pub struct MainLayout {
    activity_area_layout: ActivityAreaLayout,
    balance: f32,
}

impl MainLayout {
    pub fn init(parse_result: ParseResult) -> Self {
        MainLayout {
            activity_area_layout: ActivityAreaLayout::init(parse_result.transactions),
            balance: parse_result.balance,
        }
    }
}

impl Component for MainLayout {
    fn handle_child_events(&mut self, event: &Event) -> color_eyre::eyre::Result<()> {
        self.activity_area_layout.handle_events(event)
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

        let [title_chunk, transaction_chunk, balance_chunk] = *self.get_layout(area) else {
            panic!()
        };

        frame.render_widget(title, title_chunk);
        self.activity_area_layout.render(frame, transaction_chunk);
        frame.render_widget(balance, balance_chunk)
    }
}
