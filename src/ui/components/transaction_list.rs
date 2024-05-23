use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{List, ListState},
    Frame,
};

use crate::csv::models::Transaction;

#[derive(Debug, Default)]
pub struct TransactionList {
    transactions: Vec<Transaction>,
    transaction_list_state: ListState,
    transaction_list_lines: usize,
}

impl TransactionList {
    pub fn init(transactions: Vec<Transaction>) -> Self {
        TransactionList {
            transactions,
            ..Default::default()
        }
    }
    pub fn scroll_down(&mut self) {
        let transaction_list_max = self.transactions.len() - self.transaction_list_lines;
        let new_offset = *self.transaction_list_state.offset_mut() + 1usize;
        *self.transaction_list_state.offset_mut() = if new_offset <= transaction_list_max {
            new_offset
        } else {
            transaction_list_max
        };
    }
    pub fn scroll_up(&mut self) {
        let new_offset = if self.transaction_list_state.offset() == 0usize {
            0
        } else {
            self.transaction_list_state.offset() - 1usize
        };
        *self.transaction_list_state.offset_mut() = new_offset;
    }
    pub fn render(&mut self, frame: &mut Frame<'_>, transaction_chunk: Rect) {
        self.transaction_list_lines = transaction_chunk.rows().count();

        let list = List::new(
            self.transactions
                .iter()
                .map(|x| format!("{} - {} - {}", x.date, x.label, x.amount)),
        )
        .style(Style::default().fg(Color::Rgb(255, 176, 0)));

        frame.render_stateful_widget(list, transaction_chunk, &mut self.transaction_list_state);
    }
}
