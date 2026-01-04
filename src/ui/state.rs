use std::sync::{Arc, Mutex};

use crate::csv::{
    models::{AssignedTransaction, BudgetItem, CycleFile, Transaction},
    parsers::assigned_transactions::parse_assigned_transactions_csv,
};

#[derive(Debug)]
pub struct State {
    pub files: Arc<Mutex<Vec<CycleFile>>>,
    pub current_file: Arc<Mutex<CycleFile>>,
    pub transactions: Vec<Transaction>,
    pub blance: f32,
    pub budget_items: Vec<BudgetItem>,
    pub assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
}

impl State {
    pub fn update_current_file(&self, current_file: CycleFile) {
        let chosen_file = if current_file.path.exists() {
            current_file
        } else {
            let chosen_file = CycleFile::create_new_file().unwrap();
            self.files.lock().unwrap().push(chosen_file.clone());
            chosen_file
        };

        *self.assigned_transactions.lock().unwrap() =
            parse_assigned_transactions_csv(&chosen_file.path).unwrap();
        *self.current_file.lock().unwrap() = chosen_file;
    }
}
