mod csv;
mod ui;
mod utils;

use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use csv::{
    parsers::{assigned_transactions, budget_csv, transaction_csv},
    post_processing::{
        budget_items::add_spending_money, transactions::remove_already_processed_items,
    },
};
use ui::app_builder::{AppBuilder, State};

const ASSIGNED_TRANSACTIONS_FILE_NAME: &str = "assigned_transactions.csv";
const NEW_TRANSACTIONS_FILE_NAME: &str = "new.csv";

fn main() -> Result<()> {
    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv(ASSIGNED_TRANSACTIONS_FILE_NAME)?;
    let parse_result_result = transaction_csv::parse_transaction_csv(NEW_TRANSACTIONS_FILE_NAME);
    let mut parse_result = parse_result_result?;
    remove_already_processed_items(&mut parse_result.transactions, &assigned_transactions);

    let budget_items_result = budget_csv::parse_budget_csv("budget.csv");
    let mut budget_items = budget_items_result?;
    add_spending_money(&mut budget_items);

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;

    let state = State {
        assigned_transactions: Arc::new(Mutex::new(assigned_transactions)),
        transactions: parse_result.transactions,
        blance: parse_result.balance,
        budget_items,
    };

    AppBuilder::init()
        .create_watcher()
        .create_app(&state)?
        .run(&mut terminal)
        .map_err(|_| anyhow!("Failed to start application"))?;

    ui::wrapper::restore()?;
    Ok(())
}
