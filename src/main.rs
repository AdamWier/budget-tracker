mod csv;
mod ui;
mod utils;

use color_eyre::eyre::Result;
use csv::parsers::{assigned_transactions, budget_csv, transaction_csv};

use crate::ui::App;

fn main() -> Result<()> {
    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv("assigned_transactions.csv");
    let parse_result = transaction_csv::parse_transaction_csv("test.csv");
    let items = budget_csv::parse_budget_csv("budget.csv");

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;
    App::new(parse_result, items).run(&mut terminal)?;
    ui::wrapper::restore()?;
    Ok(())
}
