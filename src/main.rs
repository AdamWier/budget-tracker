mod csv;
mod ui;
mod utils;

use color_eyre::eyre::Result;
use csv::{
    parsers::{assigned_transactions, budget_csv, transaction_csv},
    post_processing::{
        budget_items::add_spending_money, transactions::remove_already_processed_items,
    },
};

use crate::ui::App;

fn main() -> Result<()> {
    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv("assigned_transactions.csv");
    let mut parse_result = transaction_csv::parse_transaction_csv("test.csv");
    remove_already_processed_items(&mut parse_result.transactions, &assigned_transactions);

    let mut items = budget_csv::parse_budget_csv("budget.csv");
    add_spending_money(&mut items);

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;
    App::new(parse_result, items).run(&mut terminal)?;
    ui::wrapper::restore()?;
    Ok(())
}
