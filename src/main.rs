mod consts;
mod csv;
mod start_up;
mod ui;
mod utils;

use std::{
    fs::create_dir_all,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Result};
use consts::{BUDGET_FILE_NAME, NEW_TRANSACTIONS_FILE_NAME};
use csv::{
    parsers::{assigned_transactions, budget_csv, transaction_csv},
    post_processing::budget_items::add_spending_money,
};
use ui::{app_builder::AppBuilder, state::State};

use crate::{consts::CYCLES_FOLDER, csv::models::ParseResult, start_up::get_file_list};

fn main() -> Result<()> {
    create_dir_all(CYCLES_FOLDER)?;

    let files = get_file_list()?;
    let current_file = files.last().cloned().context("No files in list")?;

    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv(&current_file.path)?;
    let ParseResult {
        transactions_to_be_assigned,
        balance,
    } = transaction_csv::parse_transaction_csv(NEW_TRANSACTIONS_FILE_NAME, &assigned_transactions)?;

    let budget_items = budget_csv::parse_budget_csv(BUDGET_FILE_NAME).map(add_spending_money)?;

    let assigned_transactions_arc = Arc::new(Mutex::new(assigned_transactions));
    let current_file_arc = Arc::new(Mutex::new(current_file));
    let files_arc = Arc::new(Mutex::new(files));

    let mut terminal = ui::wrapper::init()?;

    let state = State {
        current_file: current_file_arc,
        files: files_arc,
        assigned_transactions: assigned_transactions_arc,
        transactions: transactions_to_be_assigned,
        balance,
        budget_items,
    };

    AppBuilder::init()
        .create_assigned_transaction_watcher(&state.assigned_transactions, &state.current_file)
        .create_cycle_file_watcher(&state.files)
        .create_app(&state)?
        .run(&mut terminal)
        .map_err(|_| anyhow!("Failed to start application"))?;

    ui::wrapper::restore()?;
    Ok(())
}
