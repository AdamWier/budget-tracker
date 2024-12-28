use std::sync::{Arc, Mutex};

use crate::csv::models::{AssignedTransaction, BudgetItem, Transaction};

#[derive(Debug)]
pub struct State {
    pub transactions: Vec<Transaction>,
    pub blance: f32,
    pub budget_items: Vec<BudgetItem>,
    pub assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
}
