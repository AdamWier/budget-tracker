use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

use crate::csv::models::ParseResult;
use super::Component;

pub struct MainLayout {
    parse_result: ParseResult,
}

impl MainLayout {
    pub fn init(parse_result: &ParseResult) -> Self {
        MainLayout {
            parse_result: parse_result.clone(),
        }
    }
}

impl Component for MainLayout {
    fn render(&mut self, frame: &mut Frame<'_>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size());

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        let title = Paragraph::new(Text::styled(
            "World's Best Budget Manager",
            Style::default().fg(Color::Rgb(255, 176, 0)),
        ))
        .alignment(Alignment::Center)
        .block(block.clone());

        let list = List::new(
            self.parse_result
                .transactions
                .iter()
                .map(|x| format!("{} - {} - {}", x.date, x.label, x.amount)),
        )
        .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        let balance = Paragraph::new(Text::styled(
            format!("Balance: {}", self.parse_result.balance),
            Style::default().fg(Color::Rgb(255, 176, 0)),
        ))
        .alignment(Alignment::Center)
        .block(block.clone());

        frame.render_widget(title, chunks[0]);
        frame.render_widget(list, chunks[1]);
        frame.render_widget(balance, chunks[2])
    }
}
