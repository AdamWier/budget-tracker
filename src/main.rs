mod consts;
mod csv;
mod ui;
mod utils;

use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use consts::{ASSIGNED_TRANSACTIONS_FILE_NAME, BUDGET_FILE_NAME, NEW_TRANSACTIONS_FILE_NAME};
use csv::{
    parsers::{assigned_transactions, budget_csv, transaction_csv},
    post_processing::{
        budget_items::add_spending_money, transactions::remove_already_processed_items,
    },
};
use ui::{app_builder::AppBuilder, state::State};

fn main() -> Result<()> {
    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv(ASSIGNED_TRANSACTIONS_FILE_NAME)?;
    let mut parse_result = transaction_csv::parse_transaction_csv(NEW_TRANSACTIONS_FILE_NAME)?;
    remove_already_processed_items(&mut parse_result.transactions, &assigned_transactions);

    let mut budget_items = budget_csv::parse_budget_csv(BUDGET_FILE_NAME)?;
    add_spending_money(&mut budget_items);

    let mut terminal = ui::wrapper::init()?;

    let assigned_transactions_arc = Arc::new(Mutex::new(assigned_transactions));

    let state = State {
        assigned_transactions: assigned_transactions_arc,
        transactions: parse_result.transactions,
        blance: parse_result.balance,
        budget_items,
    };

    AppBuilder::init()
        .create_watcher(&state.assigned_transactions)
        .create_app(&state)?
        .run(&mut terminal)
        .map_err(|_| anyhow!("Failed to start application"))?;

    ui::wrapper::restore()?;
    Ok(())
}
