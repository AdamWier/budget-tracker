use std::sync::{Arc, Mutex};

use crate::csv::{
    models::{AssignedTransaction, BudgetItem, CycleFile, Transaction},
    parsers::assigned_transactions::parse_assigned_transactions_csv,
};

#[derive(Debug)]
pub struct State {
    pub files: Vec<CycleFile>,
    pub current_file: Arc<Mutex<CycleFile>>,
    pub transactions: Vec<Transaction>,
    pub blance: f32,
    pub budget_items: Vec<BudgetItem>,
    pub assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
}

impl State {
    pub fn update_current_file(&self, current_file: CycleFile) {
        *self.assigned_transactions.lock().unwrap() =
            parse_assigned_transactions_csv(&current_file.path).unwrap();
        *self.current_file.lock().unwrap() = current_file;
    }
}
