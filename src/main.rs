mod consts;
mod csv;
mod start_up;
mod ui;
mod utils;

use std::{
    fs::create_dir_all,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Result};
use consts::{BUDGET_FILE_NAME, NEW_TRANSACTIONS_FILE_NAME};
use csv::{
    parsers::{assigned_transactions, budget_csv, transaction_csv},
    post_processing::{
        budget_items::add_spending_money, transactions::remove_already_processed_items,
    },
};
use itertools::Itertools;
use ui::{app_builder::AppBuilder, state::State};

use crate::{consts::CYCLES_FOLDER, start_up::get_file_list};

fn main() -> Result<()> {
    create_dir_all(CYCLES_FOLDER)?;

    let file_list = get_file_list()?;
    let current_file = file_list.first().cloned().context("No files in list")?;

    let all_file_choices = file_list.into_iter().sorted().unique().collect_vec();

    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv(&current_file.path)?;
    let mut parse_result = transaction_csv::parse_transaction_csv(NEW_TRANSACTIONS_FILE_NAME)?;
    remove_already_processed_items(&mut parse_result.transactions, &assigned_transactions);

    let mut budget_items = budget_csv::parse_budget_csv(BUDGET_FILE_NAME)?;
    add_spending_money(&mut budget_items);

    let mut terminal = ui::wrapper::init()?;

    let assigned_transactions_arc = Arc::new(Mutex::new(assigned_transactions));
    let current_file_arc = Arc::new(Mutex::new(current_file));
    let all_files_arc = Arc::new(Mutex::new(all_file_choices));

    let state = State {
        current_file: current_file_arc,
        files: all_files_arc,
        assigned_transactions: assigned_transactions_arc,
        transactions: parse_result.transactions,
        blance: parse_result.balance,
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
