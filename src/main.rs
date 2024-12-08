mod csv;
mod ui;
mod utils;

use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use csv::{
    models::{AssignedTransaction, BudgetItem, Transaction},
    parsers::{assigned_transactions, budget_csv, transaction_csv},
    post_processing::{
        budget_items::add_spending_money, transactions::remove_already_processed_items,
    },
};

use crate::ui::App;

const ASSIGNED_TRANSACTIONS_FILE_NAME: &str = "assigned_transactions.csv";
const NEW_TRANSACTIONS_FILE_NAME: &str = "new.csv";

#[derive(Debug)]
pub struct State {
    pub transactions: Vec<Transaction>,
    pub balance: f32,
    pub budget_items: Vec<BudgetItem>,
    pub assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
}
fn main() -> Result<()> {
    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv(ASSIGNED_TRANSACTIONS_FILE_NAME)?;
    let parse_result_result = transaction_csv::parse_transaction_csv(NEW_TRANSACTIONS_FILE_NAME);
    let mut parse_result = parse_result_result?;
    remove_already_processed_items(&mut parse_result.transactions, &assigned_transactions);

    let items_result = budget_csv::parse_budget_csv("budget.csv");
    let mut items = items_result?;
    add_spending_money(&mut items);

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;
    let assigned_transactions_original = Arc::new(Mutex::new(assigned_transactions));
    App::new(parse_result, items, assigned_transactions_original)
        .run(&mut terminal)
        .map_err(|_| anyhow!("Failed to start application"))?;
    ui::wrapper::restore()?;
    Ok(())
}
