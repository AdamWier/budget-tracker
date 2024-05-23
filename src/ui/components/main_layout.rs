use std::rc::Rc;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use super::{transaction_list::TransactionList, Component};
use crate::csv::models::ParseResult;

#[derive(Debug, Default)]
pub struct MainLayout {
    balance: f32,
    transaction_list: TransactionList,
}

impl MainLayout {
    pub fn init(parse_result: ParseResult) -> Self {
        MainLayout {
            transaction_list: TransactionList::init(parse_result.transactions),
            balance: parse_result.balance,
        }
    }
}

impl Component for MainLayout {
    fn handle_key_events(&mut self, key: &crossterm::event::KeyEvent) -> () {
        match key.code {
            KeyCode::Down => self.transaction_list.scroll_down(),
            KeyCode::Up => self.transaction_list.scroll_up(),
            _ => {}
        }
    }
    fn get_layout(&mut self, frame: &mut Frame) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size())
    }
    fn render(&mut self, frame: &mut Frame<'_>) {
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

        let [title_chunk, transaction_chunk, balance_chunk] = *self.get_layout(frame) else {
            panic!()
        };

        frame.render_widget(title, title_chunk);
        self.transaction_list.render(frame, transaction_chunk);
        frame.render_widget(balance, balance_chunk)
    }
}
