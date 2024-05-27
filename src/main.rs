mod csv;
mod ui;
mod utils;

use color_eyre::eyre::Result;
use csv::{
    models::ComparableTransaction,
    parsers::{assigned_transactions, budget_csv, transaction_csv},
};

use crate::ui::App;

fn main() -> Result<()> {
    let assigned_transactions =
        assigned_transactions::parse_assigned_transactions_csv("assigned_transactions.csv");
    let mut parse_result = transaction_csv::parse_transaction_csv("test.csv");
    parse_result.transactions.retain(|x| {
        !assigned_transactions
            .iter()
            .any(|y| x.get_comparable_value() == y.get_comparable_value())
    });

    let items = budget_csv::parse_budget_csv("budget.csv");

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;
    App::new(parse_result, items).run(&mut terminal)?;
    ui::wrapper::restore()?;
    Ok(())
}
